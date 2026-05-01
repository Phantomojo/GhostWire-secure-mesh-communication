/*
 * GhostWire ESP32 Firmware - Blink Test
 * 
 * Purpose: Verify toolchain and board setup
 * Author: Phantomojo
 * Date: 2026-04-12
 * 
 * This is the first step in building ESP32 mesh capabilities
 * for GhostWire secure mesh communication.
 */

#include <stdio.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "driver/gpio.h"
#include "sdkconfig.h"

/* Can use project configuration menu (idf.py menuconfig) to change this info */
#define BLINK_GPIO GPIO_NUM_2

void app_main(void)
{
    printf("===============================\n");
    printf("GhostWire ESP32 Firmware v0.1.0\n");
    printf("===============================\n\n");
    
    /* Configure the IOMUX GPIO function of this pin */
    gpio_reset_pin(BLINK_GPIO);
    /* Set the GPIO as a push/pull output */
    gpio_set_direction(BLINK_GPIO, GPIO_MODE_OUTPUT);
    
    printf("Setup complete! Starting blink test...\n");
    printf("LED on GPIO%d should be blinking every 1 second.\n\n", BLINK_GPIO);
    
    while(1) {
        /* Blink on */
        gpio_set_level(BLINK_GPIO, 1);
        printf("[LED] ON\n");
        vTaskDelay(1000 / portTICK_PERIOD_MS);
        
        /* Blink off */
        gpio_set_level(BLINK_GPIO, 0);
        printf("[LED] OFF\n");
        vTaskDelay(1000 / portTICK_PERIOD_MS);
    }
}
