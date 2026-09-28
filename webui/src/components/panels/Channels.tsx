import React, { useState, useEffect } from 'react';
import { useApp } from '../../context/AppContext';
import { channels as channelsApi, type GhostChannel } from '../../services/api';

export default function ChannelsPanel() {
  const { toast } = useApp();
  const [channelList, setChannelList] = useState<GhostChannel[]>([]);
  const [selected, setSelected] = useState<GhostChannel | null>(null);
  const [msgs, setMsgs] = useState<string[]>([]);
  const [input, setInput] = useState('');
  const [inviteInput, setInviteInput] = useState('');
  const [loading, setLoading] = useState(false);
  const [creating, setCreating] = useState(false);
  const [newName, setNewName] = useState('');

  const load = async () => {
    const list = await channelsApi.list().catch(() => []);
    setChannelList(list);
  };

  useEffect(() => { load(); }, []);

  useEffect(() => {
    if (!selected) return;
    channelsApi.messages(selected.topic_id)
      .then(setMsgs)
      .catch(() => setMsgs([]));
  }, [selected]);

  const handleCreate = async () => {
    if (!newName.trim()) return;
    setLoading(true);
    try {
      const ch = await channelsApi.create(newName.trim());
      toast('success', `Channel ${ch.local_name ?? ch.topic_id.slice(0,8)} created`);
      setCreating(false);
      setNewName('');
      await load();
    } catch (e: any) {
      toast('error', e.message ?? 'Create failed');
    } finally { setLoading(false); }
  };

  const handleJoin = async () => {
    if (!inviteInput.trim()) return;
    setLoading(true);
    try {
      const ch = await channelsApi.join(inviteInput.trim());
      toast('success', 'Joined channel');
      setInviteInput('');
      await load();
      setSelected(ch);
    } catch (e: any) {
      toast('error', e.message ?? 'Invalid invite');
    } finally { setLoading(false); }
  };

  const handleLeave = async (ch: GhostChannel) => {
    // Was: `.catch(() => {})` followed by an unconditional
    // toast('info','Left channel'). A failed leave still reported success and
    // then cleared the selection, so the operator was told they had left a
    // channel they were still in. Surface the real error instead.
    try {
      await channelsApi.leave(ch.topic_id);
      toast('info', 'Left channel');
      if (selected?.topic_id === ch.topic_id) setSelected(null);
      await load();
    } catch (e: any) {
      toast('error', e.message ?? 'Leave failed');
    }
  };

  const handlePublish = async () => {
    if (!selected || !input.trim()) return;
    setLoading(true);
    try {
      await channelsApi.publish(selected.topic_id, input.trim());
      toast('success', 'Message published');
      setInput('');
      const updated = await channelsApi.messages(selected.topic_id).catch(() => msgs);
      setMsgs(updated);
    } catch (e: any) {
      toast('error', e.message ?? 'Publish failed — PoW may take a moment');
    } finally { setLoading(false); }
  };

  return (
    <div style={{ display: 'flex', height: '100%', overflow: 'hidden' }}>
      {/* Channel list sidebar */}
      <div style={{
        width: 220, borderRight: '1px solid var(--border)',
        display: 'flex', flexDirection: 'column', flexShrink: 0,
      }}>
        <div style={{
          padding: '12px 12px 8px',
          borderBottom: '1px solid var(--border)',
        }}>
          <div className="ops-section-title" style={{ marginBottom: 10 }}>Ghost Channels</div>

          {/* Join by invite */}
          <div style={{ display: 'flex', gap: 4, marginBottom: 6 }}>
            <input
              className="ops-input"
              placeholder="gwRSTuNCs:... invite"
              value={inviteInput}
              onChange={e => setInviteInput(e.target.value)}
              style={{ fontSize: 10 }}
            />
            <button className="btn-ops btn-ops-primary"
              onClick={handleJoin} disabled={loading}
              style={{ padding: '4px 8px', flexShrink: 0 }}>
              Join
            </button>
          </div>

          {/* Create */}
          {creating ? (
            <div style={{ display: 'flex', gap: 4 }}>
              <input
                className="ops-input"
                placeholder="channel name"
                value={newName}
                onChange={e => setNewName(e.target.value)}
                onKeyDown={e => e.key === 'Enter' && handleCreate()}
                style={{ fontSize: 10 }}
                autoFocus
              />
              <button className="btn-ops btn-ops-primary"
                onClick={handleCreate} disabled={loading}
                style={{ padding: '4px 6px', flexShrink: 0 }}>
                ✓
              </button>
              <button className="btn-ops"
                onClick={() => setCreating(false)}
                style={{ padding: '4px 6px', flexShrink: 0 }}>
                ✕
              </button>
            </div>
          ) : (
            <button className="btn-ops btn-ops-primary"
              onClick={() => setCreating(true)}
              style={{ width: '100%', justifyContent: 'center' }}>
              ◉ New channel
            </button>
          )}
        </div>

        {/* Channel list */}
        <div style={{ flex: 1, overflowY: 'auto', padding: 6 }}>
          {channelList.length === 0 ? (
            <div style={{
              padding: '20px 8px', textAlign: 'center',
              fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--t4)',
            }}>
              No channels<br/>Create or join one above
            </div>
          ) : channelList.map(ch => (
            <div
              key={ch.topic_id}
              onClick={() => setSelected(ch)}
              style={{
                padding: '8px 10px',
                borderRadius: 'var(--r-sm)',
                cursor: 'pointer',
                marginBottom: 2,
                background: selected?.topic_id === ch.topic_id
                  ? 'var(--sig-ghost)' : 'transparent',
                border: `1px solid ${selected?.topic_id === ch.topic_id
                  ? 'var(--sig-dim)' : 'transparent'}`,
                display: 'flex', alignItems: 'center', gap: 8,
              }}
            >
              <span style={{ fontSize: 10 }}>◉</span>
              <div style={{ flex: 1, overflow: 'hidden' }}>
                <div style={{
                  fontFamily: 'var(--mono)', fontSize: 11,
                  color: selected?.topic_id === ch.topic_id ? 'var(--sig)' : 'var(--t2)',
                  overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap',
                }}>
                  {ch.local_name ?? ch.topic_id.slice(0, 8)}
                </div>
                <div style={{
                  fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)',
                  overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap',
                }}>
                  {ch.topic_id.slice(0, 16)}...
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Main area */}
      {selected ? (
        <div style={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
          {/* Channel header */}
          <div style={{
            padding: '10px 16px',
            borderBottom: '1px solid var(--border)',
            display: 'flex', alignItems: 'center', gap: 12,
          }}>
            <span style={{ fontSize: 14 }}>◉</span>
            <div style={{ flex: 1 }}>
              <div style={{
                fontFamily: 'var(--mono)', fontSize: 13, fontWeight: 700,
                color: 'var(--t1)',
              }}>
                {selected.local_name ?? 'unnamed channel'}
              </div>
              <div className="genesis-id">
                ID <span>{selected.topic_id}</span>
              </div>
            </div>

            {/* Invite string */}
            <button
              className="btn-ops"
              onClick={() => {
                if (selected.invite_string) {
                  navigator.clipboard?.writeText(selected.invite_string);
                  toast('success', 'Invite copied to clipboard');
                }
              }}
              title="Copy invite string"
            >
              ⟁ Invite
            </button>
            <button className="btn-ops btn-ops-danger"
              onClick={() => handleLeave(selected)}>
              Leave
            </button>
          </div>

          {/* Privacy notice */}
          <div style={{
            padding: '8px 16px',
            background: 'var(--surface)',
            borderBottom: '1px solid var(--border)',
            display: 'flex', gap: 8, alignItems: 'center',
          }}>
            <span style={{ fontSize: 10, color: 'var(--sig-dim)' }}>⬢</span>
            <span style={{ fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--t3)' }}>
              No sender attribution · No timestamps · No read receipts · PoW anti-spam · Messages routed via Sphinx
            </span>
          </div>

          {/* Messages */}
          <div style={{ flex: 1, overflowY: 'auto', padding: '12px 16px' }}>
            {msgs.length === 0 ? (
              <div style={{
                textAlign: 'center', padding: '32px 0',
                fontFamily: 'var(--mono)', fontSize: 11, color: 'var(--t4)',
              }}>
                No messages yet.<br/>
                <span style={{ color: 'var(--t3)' }}>
                  Messages are anonymous — no one knows who sent what.
                </span>
              </div>
            ) : msgs.map((msg, i) => (
              <div key={i} style={{
                marginBottom: 10,
                padding: '8px 12px',
                background: 'var(--surface)',
                border: '1px solid var(--border)',
                borderRadius: 'var(--r-sm)',
                fontFamily: 'var(--mono)',
                fontSize: 12,
                color: 'var(--t1)',
                borderLeft: '2px solid var(--sig-dim)',
              }}>
                {msg}
              </div>
            ))}
          </div>

          {/* Compose */}
          <div style={{
            padding: '12px 16px',
            borderTop: '1px solid var(--border)',
            display: 'flex', gap: 8,
          }}>
            <input
              className="ops-input"
              placeholder={`Publish anonymously to ${selected.local_name ?? 'channel'}...`}
              value={input}
              onChange={e => setInput(e.target.value)}
              onKeyDown={e => e.key === 'Enter' && !loading && handlePublish()}
              disabled={loading}
              style={{ flex: 1 }}
            />
            <button
              className="btn-ops btn-ops-primary"
              onClick={handlePublish}
              disabled={loading || !input.trim()}
            >
              {loading ? '⟳' : '⟁ Send'}
            </button>
          </div>
          {loading && (
            <div style={{
              padding: '4px 16px 8px',
              fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--t3)',
            }}>
              Solving proof-of-work puzzle (~2s)...
            </div>
          )}
        </div>
      ) : (
        <div style={{
          flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center',
          flexDirection: 'column', gap: 8,
        }}>
          <div style={{ fontSize: 32, color: 'var(--t4)' }}>◉</div>
          <div style={{ fontFamily: 'var(--mono)', fontSize: 12, color: 'var(--t3)' }}>
            Select or create a Ghost Channel
          </div>
          <div style={{ fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--t4)', maxWidth: 320, textAlign: 'center' }}>
            Channels have no owner, no member list, no sender attribution.
            Anyone with the invite string can join.
          </div>
        </div>
      )}
    </div>
  );
}
