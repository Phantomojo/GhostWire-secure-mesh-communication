# Research Pipeline

Every agent research output follows this flow:

```
Agent completes → Save to research/ → Graphify incorporates → Knowledge base updated
```

## Convention

1. **File naming**: `research/{topic}-{date}.md` (e.g., `prphet-dtn-2026-07-25.md`)
2. **Header format**: Each file starts with metadata block:
   ```markdown
   # {Topic}
   - Date: YYYY-MM-DD
   - Agent: {agent type} (session: {session_id})
   - Status: raw | reviewed | integrated
   ```
3. **After saving**: Run `graphify update .` to incorporate into knowledge graph
4. **Graphify queries**: Use `graphify query "{question}"` to retrieve from knowledge base

## Agent Output Save Script

```bash
# Save agent output to research file
# Usage: ./save-research.sh <topic> <session_id> < content.md
TOPIC=$1
SESSION=$2
DATE=$(date +%Y-%m-%d)
FILE="research/${TOPIC}-${DATE}.md"

cat > "$FILE" <<EOF
# ${TOPIC}
- Date: ${DATE}
- Agent: background task (session: ${SESSION})
- Status: raw

---

EOF

cat >> "$FILE"
echo "Saved to ${FILE}"
echo "Run: graphify update ."
```

## Files

| File | Topic | Agent | Date |
|------|-------|-------|------|
| prphet-dtn-upgrade.md | PRoPHET DTN improvements | librarian | 2026-07-25 |
| sphinx-onion-routing.md | Sphinx originate capability | librarian | 2026-07-25 |
| nostr-mesh-protocol.md | Nostr for mesh fallback | librarian | 2026-07-25 |
| ghostwire-codebase-gaps.md | DTN/routing gap analysis | explore | 2026-07-25 |
