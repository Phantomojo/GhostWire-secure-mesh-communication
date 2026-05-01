/*
 * GhostWire ESP32 - WiFi Network Scanner
 * 
 * Purpose: Scan and display nearby WiFi networks
 * Author: Phantomojo
 * Date: 2026-04-12
 * 
 * This demonstrates ESP32's WiFi station mode scanning capability.
 * Useful for network reconnaissance and mesh planning.
 */

#include <stdio.h>
#include <string.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "freertos/event_groups.h"
#include "esp_wifi.h"
#include "esp_event.h"
#include "esp_log.h"
#include "nvs_flash.h"

static const char *TAG = "ghostwire_wifi_scanner";

/* Scan parameters */
#define SCAN_AUTH_MODE_THRESHOLD WIFI_AUTH_WPA2_PSK
#define SCAN_RSSI_THRESHOLD      -90  // Minimum signal strength
#define SCAN_CHANNEL_LIST_SIZE   14   // 2.4GHz channels 1-14

/* WiFi authentication mode names */
static const char *auth_mode_str[] = {
    "OPEN",        // 0
    "WEP",         // 1
    "WPA_PSK",     // 2
    "WPA2_PSK",    // 3
    "WPA_WPA2_PSK",// 4
    "WPA2_ENTERPRISE", // 5
    "WPA3_PSK",    // 6
    "WPA2_WPA3_PSK",   // 7
    "WAPI_PSK",    // 8
    "WPA3_ENT_192",    // 9
    "WPA3_EXT_PSK",    // 10
    "WPA3_EXT_PSK_MIX11", // 11
};

/* Print scan results in a nice table */
static void print_scan_results(uint16_t number, wifi_ap_record_t *ap_records)
{
    printf("\n");
    printf("=============================================================\n");
    printf("  GhostWire WiFi Network Scanner\n");
    printf("=============================================================\n");
    printf("  Found %d networks\n", number);
    printf("=============================================================\n\n");
    
    printf("%-4s %-33s %-13s %-6s %-5s %-8s\n", 
           "ID", "SSID", "BSSID (MAC)", "RSSI", "CH", "AUTH");
    printf("---- --------------------------------- ------------- ------ ----- --------\n");
    
    for (int i = 0; i < number; i++) {
        wifi_ap_record_t *ap = &ap_records[i];
        
        // Skip weak signals
        if (ap->rssi < SCAN_RSSI_THRESHOLD) {
            continue;
        }
        
        // Format BSSID
        char bssid_str[18];
        snprintf(bssid_str, sizeof(bssid_str), "%02x:%02x:%02x:%02x:%02x:%02x",
                 ap->bssid[0], ap->bssid[1], ap->bssid[2],
                 ap->bssid[3], ap->bssid[4], ap->bssid[5]);
        
        // Get auth mode string
        const char *auth_str = "UNKNOWN";
        if (ap->authmode < sizeof(auth_mode_str) / sizeof(auth_mode_str[0])) {
            auth_str = auth_mode_str[ap->authmode];
        }
        
        // Print network info
        printf("%-4d %-32s  %-13s %-6d %-5d %-8s\n",
               i + 1,
               (char *)ap->ssid,
               bssid_str,
               ap->rssi,
               ap->primary,
               auth_str);
    }
    
    printf("\n=============================================================\n");
    printf("  RSSI: Signal strength (dBm)\n");
    printf("    -30 to -50: Excellent\n");
    printf("    -50 to -70: Good\n");
    printf("    -70 to -90: Fair\n");
    printf("    -90+:       Weak\n");
    printf("=============================================================\n\n");
}

/* Perform WiFi scan */
static void wifi_scan(void)
{
    wifi_ap_record_t *ap_records = NULL;
    uint16_t ap_count = 0;
    esp_err_t ret;
    
    printf("\n🔍 Starting WiFi scan...\n\n");
    
    // Get AP count
    ret = esp_wifi_scan_get_ap_num(&ap_count);
    if (ret != ESP_OK) {
        ESP_LOGE(TAG, "Failed to get AP count: %s", esp_err_to_name(ret));
        return;
    }
    
    if (ap_count == 0) {
        printf("No networks found!\n");
        return;
    }
    
    ESP_LOGI(TAG, "Found %d networks, allocating memory...", ap_count);
    
    // Allocate memory for AP records
    ap_records = malloc(sizeof(wifi_ap_record_t) * ap_count);
    if (ap_records == NULL) {
        ESP_LOGE(TAG, "Failed to allocate memory for AP records");
        return;
    }
    
    // Get AP records
    ret = esp_wifi_scan_get_ap_records(&ap_count, ap_records);
    if (ret != ESP_OK) {
        ESP_LOGE(TAG, "Failed to get AP records: %s", esp_err_to_name(ret));
        free(ap_records);
        return;
    }
    
    // Print results
    print_scan_results(ap_count, ap_records);
    
    // Free memory
    free(ap_records);
}

/* WiFi scan configuration */
static void wifi_scan_config(void)
{
    wifi_scan_config_t scan_config = {
        .ssid = NULL,
        .bssid = NULL,
        .channel = 0,           // Scan all channels
        .show_hidden = true,    // Include hidden networks
        .scan_type = WIFI_SCAN_TYPE_ACTIVE,
        .scan_time.active.min = 100,
        .scan_time.active.max = 300,
    };
    
    ESP_ERROR_CHECK(esp_wifi_scan_start(&scan_config, true));
    ESP_LOGI(TAG, "Scan completed successfully");
}

void app_main(void)
{
    // Initialize NVS (required for WiFi)
    esp_err_t ret = nvs_flash_init();
    if (ret == ESP_ERR_NVS_NO_FREE_PAGES || ret == ESP_ERR_NVS_NEW_VERSION_FOUND) {
        ESP_ERROR_CHECK(nvs_flash_erase());
        ret = nvs_flash_init();
    }
    ESP_ERROR_CHECK(ret);
    
    // Initialize WiFi
    ESP_ERROR_CHECK(esp_netif_init());
    ESP_ERROR_CHECK(esp_event_loop_create_default());
    
    wifi_init_config_t cfg = WIFI_INIT_CONFIG_DEFAULT();
    ESP_ERROR_CHECK(esp_wifi_init(&cfg));
    
    // Set WiFi mode to station
    ESP_ERROR_CHECK(esp_wifi_set_mode(WIFI_MODE_STA));
    ESP_ERROR_CHECK(esp_wifi_start());
    
    printf("===============================\n");
    printf("GhostWire WiFi Scanner v1.0.0\n");
    printf("===============================\n\n");
    
    // Perform initial scan
    wifi_scan_config();
    wifi_scan();
    
    // Scan every 30 seconds
    while (1) {
        printf("⏳ Next scan in 30 seconds...\n\n");
        vTaskDelay(30000 / portTICK_PERIOD_MS);
        
        wifi_scan_config();
        wifi_scan();
    }
}
