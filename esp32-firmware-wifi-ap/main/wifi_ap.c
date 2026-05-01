/*
 * GhostWire ESP32 - WiFi Access Point Mode
 * 
 * Purpose: Create a GhostWire WiFi hotspot
 * Author: Phantomojo
 * Date: 2026-04-12
 * 
 * This demonstrates ESP32's AP mode capability.
 * The ESP32 becomes a WiFi access point that devices can connect to.
 * Useful for offline mesh communication and device-to-device networking.
 */

#include <stdio.h>
#include <string.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "freertos/event_groups.h"
#include "esp_wifi.h"
#include "esp_event.h"
#include "esp_log.h"
#include "esp_system.h"
#include "nvs_flash.h"
#include "lwip/err.h"
#include "lwip/sys.h"

static const char *TAG = "ghostwire_wifi_ap";

/* AP Configuration */
#define GHOSTWIRE_AP_SSID       "GhostWire-Mesh"
#define GHOSTWIRE_AP_PASSWORD   "GhostWire2026"
#define GHOSTWIRE_AP_CHANNEL    6
#define GHOSTWIRE_AP_MAX_CONN   4
#define GHOSTWIRE_AP_AUTH_MODE  WIFI_AUTH_WPA2_PSK

/* Network configuration */
#define GHOSTWIRE_AP_IP         "10.0.0.1"
#define GHOSTWIRE_AP_NETMASK    "255.255.255.0"
#define GHOSTWIRE_AP_GW         "10.0.0.1"

/* Event handler for WiFi events */
static void wifi_event_handler(void* arg, esp_event_base_t event_base,
                                int32_t event_id, void* event_data)
{
    if (event_base == WIFI_EVENT && event_id == WIFI_EVENT_AP_STACONNECTED) {
        wifi_event_ap_staconnected_t* event = (wifi_event_ap_staconnected_t*) event_data;
        ESP_LOGI(TAG, "📱 Device connected: MAC=%02x:%02x:%02x:%02x:%02x:%02x, AID=%d",
                 event->mac[0], event->mac[1], event->mac[2],
                 event->mac[3], event->mac[4], event->mac[5],
                 event->aid);
    } else if (event_base == WIFI_EVENT && event_id == WIFI_EVENT_AP_STADISCONNECTED) {
        wifi_event_ap_stadisconnected_t* event = (wifi_event_ap_stadisconnected_t*) event_data;
        ESP_LOGI(TAG, "📴 Device disconnected: MAC=%02x:%02x:%02x:%02x:%02x:%02x, AID=%d",
                 event->mac[0], event->mac[1], event->mac[2],
                 event->mac[3], event->mac[4], event->mac[5],
                 event->aid);
    } else if (event_base == IP_EVENT && event_id == IP_EVENT_AP_STAIPASSIGNED) {
        ESP_LOGI(TAG, "✅ IP address assigned to station");
    }
}

/* Initialize WiFi Access Point */
static void wifi_init_ap(void)
{
    // Initialize network interface
    ESP_ERROR_CHECK(esp_netif_init());
    
    // Create default event loop
    ESP_ERROR_CHECK(esp_event_loop_create_default());
    
    // Create AP network interface
    esp_netif_create_default_wifi_ap();
    
    // Initialize WiFi
    wifi_init_config_t cfg = WIFI_INIT_CONFIG_DEFAULT();
    ESP_ERROR_CHECK(esp_wifi_init(&cfg));
    
    // Register event handlers
    ESP_ERROR_CHECK(esp_event_handler_instance_register(WIFI_EVENT,
                                                        ESP_EVENT_ANY_ID,
                                                        &wifi_event_handler,
                                                        NULL,
                                                        NULL));
    ESP_ERROR_CHECK(esp_event_handler_instance_register(IP_EVENT,
                                                        ESP_EVENT_ANY_ID,
                                                        &wifi_event_handler,
                                                        NULL,
                                                        NULL));
    
    // Configure AP
    wifi_config_t wifi_config = {
        .ap = {
            .ssid = GHOSTWIRE_AP_SSID,
            .ssid_len = strlen(GHOSTWIRE_AP_SSID),
            .channel = GHOSTWIRE_AP_CHANNEL,
            .password = GHOSTWIRE_AP_PASSWORD,
            .max_connection = GHOSTWIRE_AP_MAX_CONN,
            .authmode = GHOSTWIRE_AP_AUTH_MODE,
            .ssid_hidden = 0,  // 0 = visible, 1 = hidden
        },
    };
    
    // Start WiFi in AP mode
    ESP_ERROR_CHECK(esp_wifi_set_mode(WIFI_MODE_AP));
    ESP_ERROR_CHECK(esp_wifi_set_config(WIFI_IF_AP, &wifi_config));
    ESP_ERROR_CHECK(esp_wifi_start());
    
    ESP_LOGI(TAG, "WiFi AP started successfully");
}

/* Print AP status */
static void print_ap_status(void)
{
    printf("\n");
    printf("=============================================================\n");
    printf("  🌐 GhostWire WiFi Access Point\n");
    printf("=============================================================\n\n");
    printf("  SSID:            %s\n", GHOSTWIRE_AP_SSID);
    printf("  Password:        %s\n", GHOSTWIRE_AP_PASSWORD);
    printf("  Channel:         %d\n", GHOSTWIRE_AP_CHANNEL);
    printf("  Max Connections: %d\n", GHOSTWIRE_AP_MAX_CONN);
    printf("  Security:        WPA2-PSK\n");
    printf("  Network:         10.0.0.0/24\n");
    printf("  Gateway:         10.0.0.1\n");
    printf("\n");
    printf("  📡 Scan for 'GhostWire-Mesh' on your phone/laptop\n");
    printf("  🔗 Connect with password: GhostWire2026\n");
    printf("\n");
    printf("=============================================================\n");
    printf("  Use Cases:\n");
    printf("  - Offline device-to-device communication\n");
    printf("  - Mesh network gateway\n");
    printf("  - Local web server hosting\n");
    printf("  - IoT sensor network hub\n");
    printf("=============================================================\n\n");
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
    
    printf("===============================\n");
    printf("GhostWire WiFi AP v1.0.0\n");
    printf("===============================\n\n");
    
    // Initialize WiFi AP
    wifi_init_ap();
    
    // Print AP status
    print_ap_status();
    
    // Monitor connections
    while (1) {
        wifi_sta_list_t wifi_sta_list;
        memset(&wifi_sta_list, 0, sizeof(wifi_sta_list));
        
        esp_wifi_ap_get_sta_list(&wifi_sta_list);
        
        printf("📊 Connected devices: %d/%d\n", 
               wifi_sta_list.num, GHOSTWIRE_AP_MAX_CONN);
        
        for (int i = 0; i < wifi_sta_list.num; i++) {
            wifi_sta_info_t *sta = &wifi_sta_list.sta[i];
            printf("  Device %d: MAC=%02x:%02x:%02x:%02x:%02x:%02x, RSSI=%d\n",
                   i + 1,
                   sta->mac[0], sta->mac[1], sta->mac[2],
                   sta->mac[3], sta->mac[4], sta->mac[5],
                   sta->rssi);
        }
        
        if (wifi_sta_list.num == 0) {
            printf("  (No devices connected)\n");
        }
        
        printf("\n");
        vTaskDelay(10000 / portTICK_PERIOD_MS);
    }
}
