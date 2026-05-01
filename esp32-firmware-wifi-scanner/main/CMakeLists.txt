idf_component_register(SRCS "wifi_scanner.c"
                    INCLUDE_DIRS "."
                    REQUIRES esp_wifi esp_event nvs_flash)
