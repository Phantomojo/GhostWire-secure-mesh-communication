/*
 * GhostWire ESP32 - BLE Scanner
 * 
 * Purpose: Scan and display nearby BLE (Bluetooth Low Energy) devices
 * Author: Phantomojo
 * Date: 2026-04-12
 * 
 * This demonstrates ESP32's BLE capability.
 * Useful for device discovery, proximity detection, and mesh planning.
 */

#include <stdio.h>
#include <string.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "freertos/event_groups.h"
#include "esp_log.h"
#include "nvs_flash.h"
#include "esp_bt.h"
#include "esp_bt_main.h"
#include "esp_gap_ble_api.h"
#include "esp_gattc_api.h"
#include "esp_gatt_defs.h"
#include "esp_bt_defs.h"

static const char *TAG = "ghostwire_ble_scanner";

/* Scan configuration */
#define BLE_SCAN_DURATION       5    // seconds per scan
#define BLE_SCAN_INTERVAL       10   // seconds between scans
#define BLE_SCAN_WINDOW         0x10 // scan window (in 0.625ms units)
#define BLE_SCAN_INTERVAL_UNITS 0x10 // scan interval (in 0.625ms units)

/* Device list */
#define MAX_BLE_DEVICES         50

typedef struct {
    esp_bd_addr_t bda;
    int8_t rssi;
    char name[32];
    uint8_t adv_data_len;
    uint8_t adv_data[64];
    bool is_connectable;
    uint8_t flag;
} ble_device_t;

static ble_device_t ble_devices[MAX_BLE_DEVICES];
static uint16_t device_count = 0;

/* Common BLE service UUIDs */
static const char *get_service_name(uint16_t uuid)
{
    switch (uuid) {
        case 0x1800: return "Generic Access";
        case 0x1801: return "Generic Attribute";
        case 0x1802: return "Immediate Alert";
        case 0x1803: return "Link Loss";
        case 0x1804: return "Tx Power";
        case 0x1805: return "Current Time";
        case 0x1809: return "Health Thermometer";
        case 0x180A: return "Device Information";
        case 0x180D: return "Heart Rate";
        case 0x180F: return "Battery Service";
        case 0x1812: return "Human Interface Device";
        case 0x181A: return "Environmental Sensing";
        case 0x181B: return "Body Composition";
        case 0x181D: return "User Data";
        case 0x181E: return "Weight Scale";
        case 0x181F: return "Bond Management";
        case 0x1820: return "Cycling Power";
        case 0x1821: return "Cycling Speed & Cadence";
        case 0xFEAA: return "Eddystone (Google)";
        case 0xFE0F: return "Philips Hue";
        case 0xFD6F: return "Samsung Fast Pair";
        case 0xFDFE: return "Google Nearby";
        default:     return "Unknown";
    }
}

/* Get device type from advertising data */
static const char *get_device_type(uint8_t *adv_data, uint8_t len)
{
    // Check for common device types in advertising data
    for (int i = 0; i < len - 2; ) {
        uint8_t field_len = adv_data[i];
        if (field_len == 0) break;
        
        uint8_t field_type = adv_data[i + 1];
        
        // Flags field
        if (field_type == 0x01 && field_len >= 2) {
            uint8_t flags = adv_data[i + 2];
            if (flags & 0x06) { // LE General Discoverable Mode
                return "BLE Device";
            }
        }
        
        // Complete Local Name
        if (field_type == 0x09) {
            // Name is present, will be extracted separately
        }
        
        // Service UUIDs
        if (field_type == 0x02 || field_type == 0x03) { // 16-bit UUIDs
            if (field_len >= 3) {
                uint16_t uuid = adv_data[i + 2] | (adv_data[i + 3] << 8);
                if (uuid == 0x180D) return "Heart Rate Monitor";
                if (uuid == 0x180F) return "Battery Device";
                if (uuid == 0x1809) return "Temperature Sensor";
                if (uuid == 0x1812) return "HID Device (Keyboard/Mouse)";
                if (uuid == 0x181A) return "Environmental Sensor";
                if (uuid == 0x181E) return "Weight Scale";
            }
        }
        
        i += field_len + 1;
    }
    
    return "BLE Device";
}

/* Extract device name from advertising data */
static void extract_device_name(uint8_t *adv_data, uint8_t len, char *name, size_t name_len)
{
    name[0] = '\0';
    
    for (int i = 0; i < len - 2; ) {
        uint8_t field_len = adv_data[i];
        if (field_len == 0) break;
        
        uint8_t field_type = adv_data[i + 1];
        
        // Complete Local Name (0x09) or Shortened Local Name (0x08)
        if ((field_type == 0x09 || field_type == 0x08) && field_len > 1) {
            uint8_t name_len_actual = field_len - 1;
            if (name_len_actual >= name_len) {
                name_len_actual = name_len - 1;
            }
            memcpy(name, &adv_data[i + 2], name_len_actual);
            name[name_len_actual] = '\0';
            return;
        }
        
        i += field_len + 1;
    }
}

/* Check if device already exists */
static bool device_exists(esp_bd_addr_t bda)
{
    for (int i = 0; i < device_count; i++) {
        if (memcmp(ble_devices[i].bda, bda, ESP_BD_ADDR_LEN) == 0) {
            return true;
        }
    }
    return false;
}

/* Add device to list */
static void add_device(esp_ble_gap_cb_param_t *scan_result)
{
    if (device_count >= MAX_BLE_DEVICES) {
        return;
    }
    
    if (device_exists(scan_result->scan_rst.bda)) {
        return;
    }
    
    memcpy(ble_devices[device_count].bda, scan_result->scan_rst.bda, ESP_BD_ADDR_LEN);
    ble_devices[device_count].rssi = scan_result->scan_rst.rssi;
    ble_devices[device_count].adv_data_len = scan_result->scan_rst.adv_data_len;
    // Assume connectable if we have advertising data
    ble_devices[device_count].is_connectable = (scan_result->scan_rst.adv_data_len > 0);
    
    // Extract name
    extract_device_name(scan_result->scan_rst.ble_adv, 
                       scan_result->scan_rst.adv_data_len,
                       ble_devices[device_count].name,
                       sizeof(ble_devices[device_count].name));
    
    device_count++;
}

/* Print scan results */
static void print_ble_devices(void)
{
    printf("\n");
    printf("=============================================================\n");
    printf("  📡 GhostWire BLE Device Scanner\n");
    printf("=============================================================\n");
    printf("  Found %d BLE devices\n", device_count);
    printf("=============================================================\n\n");
    
    if (device_count == 0) {
        printf("  No BLE devices found!\n");
        printf("  Make sure BLE devices are nearby and advertising.\n\n");
        return;
    }
    
    printf("%-4s %-18s %-20s %-6s %-15s %s\n", 
           "ID", "MAC Address", "Name", "RSSI", "Type", "Connect");
    printf("---- ------------------ -------------------- ------ --------------- --------\n");
    
    for (int i = 0; i < device_count; i++) {
        ble_device_t *dev = &ble_devices[i];
        
        const char *device_type = get_device_type(dev->adv_data, dev->adv_data_len);
        const char *connect_str = dev->is_connectable ? "✅ Yes" : "❌ No";
        
        printf("%-4d %02x:%02x:%02x:%02x:%02x:%02x %-20s %-6d %-15s %s\n",
               i + 1,
               dev->bda[0], dev->bda[1], dev->bda[2],
               dev->bda[3], dev->bda[4], dev->bda[5],
               strlen(dev->name) > 0 ? dev->name : "(unknown)",
               dev->rssi,
               device_type,
               connect_str);
    }
    
    printf("\n=============================================================\n");
    printf("  RSSI: Signal strength (dBm)\n");
    printf("    -30 to -50: Excellent (very close)\n");
    printf("    -50 to -70: Good (nearby)\n");
    printf("    -70 to -90: Fair (across room)\n");
    printf("    -90+:       Weak (far away)\n");
    printf("\n");
    printf("  Common BLE Devices:\n");
    printf("  - Smartphones, tablets, laptops\n");
    printf("  - Headphones, earbuds, speakers\n");
    printf("  - Smartwatches, fitness trackers\n");
    printf("  - BLE beacons, sensors, IoT devices\n");
    printf("=============================================================\n\n");
}

/* BLE GAP event handler */
static void esp_gap_cb(esp_gap_ble_cb_event_t event, esp_ble_gap_cb_param_t *param)
{
    switch (event) {
        case ESP_GAP_BLE_SCAN_START_COMPLETE_EVT:
            if (param->scan_start_cmpl.status != ESP_BT_STATUS_SUCCESS) {
                ESP_LOGE(TAG, "Scan start failed: %d", param->scan_start_cmpl.status);
            } else {
                ESP_LOGI(TAG, "BLE scan started");
            }
            break;
            
        case ESP_GAP_BLE_SCAN_RESULT_EVT: {
            if (param->scan_rst.search_evt == ESP_GAP_SEARCH_INQ_RES_EVT) {
                add_device(param);
            } else if (param->scan_rst.search_evt == ESP_GAP_SEARCH_INQ_CMPL_EVT) {
                ESP_LOGI(TAG, "BLE scan completed, found %d devices", device_count);
                print_ble_devices();
            }
            break;
        }
        
        case ESP_GAP_BLE_SCAN_STOP_COMPLETE_EVT:
            ESP_LOGI(TAG, "BLE scan stopped");
            break;
            
        default:
            break;
    }
}

/* Configure BLE scan parameters */
static void ble_scan_config(void)
{
    esp_ble_scan_params_t ble_scan_params = {
        .scan_type = BLE_SCAN_TYPE_ACTIVE,
        .own_addr_type = BLE_ADDR_TYPE_PUBLIC,
        .scan_filter_policy = BLE_SCAN_FILTER_ALLOW_ALL,
        .scan_interval = BLE_SCAN_INTERVAL_UNITS,
        .scan_window = BLE_SCAN_WINDOW,
        .scan_duplicate = BLE_SCAN_DUPLICATE_ENABLE
    };
    
    ESP_ERROR_CHECK(esp_ble_gap_set_scan_params(&ble_scan_params));
}

/* Start BLE scan */
static void start_ble_scan(void)
{
    device_count = 0;
    memset(ble_devices, 0, sizeof(ble_devices));
    
    printf("\n🔍 Starting BLE scan (%d seconds)...\n\n", BLE_SCAN_DURATION);
    
    ESP_ERROR_CHECK(esp_ble_gap_start_scanning(BLE_SCAN_DURATION));
}

void app_main(void)
{
    // Initialize NVS
    esp_err_t ret = nvs_flash_init();
    if (ret == ESP_ERR_NVS_NO_FREE_PAGES || ret == ESP_ERR_NVS_NEW_VERSION_FOUND) {
        ESP_ERROR_CHECK(nvs_flash_erase());
        ret = nvs_flash_init();
    }
    ESP_ERROR_CHECK(ret);
    
    // Enable Bluetooth
    ESP_ERROR_CHECK(esp_bt_controller_mem_release(ESP_BT_MODE_CLASSIC_BT));
    
    esp_bt_controller_config_t bt_cfg = BT_CONTROLLER_INIT_CONFIG_DEFAULT();
    ret = esp_bt_controller_init(&bt_cfg);
    if (ret) {
        ESP_LOGE(TAG, "Bluetooth controller init failed: %d", ret);
        return;
    }
    
    ret = esp_bt_controller_enable(ESP_BT_MODE_BLE);
    if (ret) {
        ESP_LOGE(TAG, "Bluetooth controller enable failed: %d", ret);
        return;
    }
    
    ret = esp_bluedroid_init();
    if (ret) {
        ESP_LOGE(TAG, "Bluedroid init failed: %d", ret);
        return;
    }
    
    ret = esp_bluedroid_enable();
    if (ret) {
        ESP_LOGE(TAG, "Bluedroid enable failed: %d", ret);
        return;
    }
    
    // Register GAP callback
    ESP_ERROR_CHECK(esp_ble_gap_register_callback(esp_gap_cb));
    
    printf("===============================\n");
    printf("GhostWire BLE Scanner v1.0.0\n");
    printf("===============================\n\n");
    
    // Configure and start scan
    ble_scan_config();
    start_ble_scan();
    
    // Repeat scans
    while (1) {
        printf("⏳ Next scan in %d seconds...\n\n", BLE_SCAN_INTERVAL);
        vTaskDelay(BLE_SCAN_INTERVAL * 1000 / portTICK_PERIOD_MS);
        
        start_ble_scan();
    }
}
