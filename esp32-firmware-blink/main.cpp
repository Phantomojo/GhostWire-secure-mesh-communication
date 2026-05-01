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

// LED pin (most ESP32 boards have built-in LED on GPIO2)
#define LED_PIN 2

// Blink interval in milliseconds
#define BLINK_INTERVAL 1000

void setup() {
  // Initialize serial communication at 115200 baud
  Serial.begin(115200);
  Serial.println();
  Serial.println("===============================");
  Serial.println("GhostWire ESP32 Firmware v0.1.0");
  Serial.println("===============================");
  Serial.println();
  
  // Print chip information
  Serial.printf("Chip Model: %s\n", ESP.getChipModel());
  Serial.printf("Chip Revision: %d\n", ESP.getChipRevision());
  Serial.printf("CPU Frequency: %d MHz\n", ESP.getCpuFreqMHz());
  Serial.printf("Flash Size: %d bytes\n", ESP.getFlashChipSize());
  Serial.printf("Free Heap: %d bytes\n", ESP.getFreeHeap());
  Serial.printf("MAC Address: %s\n", WiFi.macAddress().c_str());
  Serial.println();
  
  // Initialize LED pin as output
  pinMode(LED_PIN, OUTPUT);
  digitalWrite(LED_PIN, LOW);
  
  Serial.println("Setup complete! Starting blink test...");
  Serial.println("LED should be blinking every 1 second.");
  Serial.println();
}

void loop() {
  // Turn LED ON
  digitalWrite(LED_PIN, HIGH);
  Serial.println("[LED] ON");
  delay(BLINK_INTERVAL);
  
  // Turn LED OFF
  digitalWrite(LED_PIN, LOW);
  Serial.println("[LED] OFF");
  delay(BLINK_INTERVAL);
}
