# The Knowledge Ark: A GhostWire Survival Feature

## What is it?
The **Knowledge Ark** transforms GhostWire from a simple communication tool into a **Civilization Backup System**.
Imagine if the internet disappeared tomorrow. How would you rebuild a generator? Treat an infection? Teach math?
The Knowledge Ark allows dedicated GhostWire nodes to host **offline copies of the world's most critical information** (Wikipedia, Medical Guides, Engineering Specs) and serve them to any device connected to the mesh.

## What's Inside? (The "Seed Config")
We propose a tiered "Ark" system based on node storage capacity:

### Tier 1: The Lifeboat (4GB - fits on any phone)
*   **Medical:** "Where There Is No Doctor" (Basic medical guide).
*   **Survival:** US Army Survival Manuals (Shelter, Water, Food).
*   **Engineering:** Low-tech mechanical guides (Pumps, simple motors).
*   **Maps:** Local OpenStreetMap vector data (High detail for immediate area).

### Tier 2: The Village Library (32GB - fits on cheap SD card)
*   **Encyclopedia:** Wikipedia (Top 50,000 articles, no images).
*   **Education:** Khan Academy Lite (Math, Science, Physics basics).
*   **Code:** StackOverflow (Selected offline dataset for rebuilding software).
*   **Literature:** Project Gutenberg selection (Philosophy, History).

### Tier 3: The Civilization Seed (128GB+ - Dedicated Node/SSD)
*   **Full Wikipedia:** Entire English Wikipedia with images (~100GB).
*   **Full Khan Academy:** All video lectures and exercises.
*   **Scientific Journals:** Public Library of Science (PLOS) datasets.
*   **Proprietary Manuals:** Repair guides for common machinery (Toyota, Honda, Solar inverters).

## How Big is It?
*   **ZIM Format:** We use the open standard `.zim` format (same as Kiwix). It offers incredible compression (approx. 50% of HTML size).
*   **Scalability:** A user doesn't need to download the whole thing. A `Tier 3` node acts as a local **Web Server**. A user on a phone just connects to the mesh and browses `http://ark.local` as if they were on the real internet.

## Why GhostWire Needs It?
1.  **Usefulness Beyond Chat:** Most people don't chat 24/7. But they *always* need information. This gives people a reason to keep their GhostWire nodes running even when not messaging.
2.  **Incentive Structure:** Nodes that host the Ark effectively become "Internet Service Providers" for the local mesh. In a future crypto-incentivized version, they could earn tokens for serving articles.
3.  **Red Bull Pitch Alignment:** It perfectly fits the "Basement" theme of "Innovation for Impact". It's not just cool tech; it's a humanitarian tool.

## Technical Implementation (Inspiration)
*   **Protocol:** We can implement a lightweight HTTP proxy in GhostWire that routes requests for `*.zim` content to the nearest node holding that file.
*   **Discovery:** The DHT (Distributed Hash Table) updates to say "Node A has `wikipedia_en_biology.zim`".
*   **Chunking:** Large files are split into chunks so multiple nodes can host parts of the library (BitTorrent style).

## Design Inspiration
*   **The Voyager Golden Record:** A curated selection of humanity.
*   **Svalbard Global Seed Vault:** But for data.
*   **IPFS (InterPlanetary File System):** For the distributed addressing logic.
