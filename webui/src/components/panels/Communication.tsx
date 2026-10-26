import React, { useState, useEffect, useRef } from 'react';
import { useApp } from '../../context/AppContext';
import { messages as msgApi, type MessageV2, type PeerV2 } from '../../services/api';

function fmt(ts: number) {
  return new Date(ts * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

function PeerItem({ peer, active, onClick }: { peer: PeerV2 | null; active: boolean; onClick: () => void }) {
  const name = peer ? (peer.nickname ?? peer.username ?? peer.node_id.slice(0, 8)) : 'ALL NODES';
  const online = peer ? peer.status === 'online' : true;
  return (
    <button onClick={onClick} style={{
      display: 'flex', alignItems: 'center', gap: 8, width: '100%',
      padding: '8px 12px', background: active ? 'var(--sig-ghost)' : 'none',
      border: `1px solid ${active ? 'var(--sig-dim)' : 'transparent'}`,
      borderRadius: 'var(--r-sm)', cursor: 'pointer', textAlign: 'left',
      marginBottom: 2, transition: 'all 0.12s',
    }}>
      <div style={{ width:6,height:6,borderRadius:'50%',flexShrink:0,background:online?'var(--sig)':'var(--t4)',boxShadow:online?'var(--sig-glow)':'none' }} />
      <div style={{ flex:1,minWidth:0 }}>
        <div style={{ fontFamily:'var(--mono)',fontSize:11,fontWeight:active?700:400,color:active?'var(--sig)':'var(--t2)',overflow:'hidden',textOverflow:'ellipsis',whiteSpace:'nowrap' }}>{name}</div>
        {peer && <div style={{ fontFamily:'var(--mono)',fontSize:9,color:'var(--t4)',marginTop:1 }}>{peer.trust_level}</div>}
      </div>
    </button>
  );
}

export default function CommunicationPanel() {
  const { state, dispatch, sendMessage } = useApp();
  const [input, setInput] = useState('');
  const [sending, setSending] = useState(false);
  const [deliveryState, setDeliveryState] = useState<'idle' | 'sending' | 'accepted' | 'failed'>('idle');
  const [deliveryError, setDeliveryError] = useState<string | null>(null);
  const [localMsgs, setLocalMsgs] = useState<MessageV2[]>([]);
  const [loading, setLoading] = useState(false);
  const endRef = useRef<HTMLDivElement>(null);
  const myId = state.profile?.id ?? '';
  const recipient = state.activeRecipient;

  useEffect(() => {
    setLoading(true);
    msgApi.list(recipient === 'broadcast' ? 'all' : recipient)
      .then(setLocalMsgs).catch(() => setLocalMsgs([]))
      .finally(() => setLoading(false));
  }, [recipient]);

  useEffect(() => {
    const relevant = state.messages.filter(m =>
      recipient === 'broadcast'
        ? m.recipient === 'broadcast'
        : (m.recipient !== 'broadcast' && (m.sender === recipient || m.recipient === recipient || m.sender === myId))
    );
    if (relevant.length > 0) {
      setLocalMsgs(prev => {
        const ids = new Set(prev.map(m => m.id));
        return [...prev, ...relevant.filter(m => !ids.has(m.id))];
      });
    }
  }, [state.messages]);

  useEffect(() => { endRef.current?.scrollIntoView({ behavior: 'smooth' }); }, [localMsgs]);

  const handleSend = async () => {
    const text = input.trim();
    if (!text || sending) return;
    setSending(true);
    setDeliveryState('sending');
    setDeliveryError(null);
    try {
      await sendMessage(text, recipient);
      setInput('');
      const updated = await msgApi.list(recipient === 'broadcast' ? 'all' : recipient).catch(() => localMsgs);
      setLocalMsgs(updated);
      setDeliveryState('accepted');
    } catch (error) {
      setDeliveryState('failed');
      setDeliveryError(error instanceof Error ? error.message : 'Message was not accepted');
    } finally { setSending(false); }
  };

  const handleKey = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); handleSend(); }
  };

  const activePeer = state.peers.find(p => p.id === recipient) ?? null;
  const activeName = recipient === 'broadcast' ? 'ALL NODES — BROADCAST'
    : (activePeer?.nickname ?? activePeer?.username ?? recipient.slice(0,12)+'...');

  return (
    <div className="split-layout">
      <div className="split-sidebar" style={{ background:'var(--surface)' }}>
        <div style={{ padding:'10px 12px',borderBottom:'1px solid var(--border)' }}>
          <div className="ops-label" style={{ marginBottom:6 }}>Direct comms</div>
        </div>
        <div style={{ flex:1,overflowY:'auto',padding:6 }}>
          <PeerItem peer={null} active={recipient==='broadcast'} onClick={() => dispatch({ type:'SET_ACTIVE_RECIPIENT',payload:'broadcast' })} />
          {state.peers.length === 0 ? (
            <div style={{ padding:'16px 8px',fontFamily:'var(--mono)',fontSize:10,color:'var(--t4)',textAlign:'center' }}>No peers discovered</div>
          ) : state.peers.map(p => (
            <PeerItem key={p.id} peer={p} active={recipient===p.id} onClick={() => dispatch({ type:'SET_ACTIVE_RECIPIENT',payload:p.id })} />
          ))}
        </div>
      </div>

      <div className="split-main">
        <div style={{ padding:'10px 16px',borderBottom:'1px solid var(--border)',display:'flex',alignItems:'center',gap:10,flexShrink:0,background:'var(--surface)' }}>
          <div style={{ width:6,height:6,borderRadius:'50%',background:'var(--sig)',boxShadow:'var(--sig-glow)',flexShrink:0 }} />
          <div style={{ flex:1,minWidth:0 }}>
            <div style={{ fontFamily:'var(--mono)',fontSize:12,fontWeight:700,color:'var(--t1)' }}>{activeName}</div>
            <div style={{ fontFamily:'var(--mono)',fontSize:9,color:'var(--t3)' }}>ChaCha20-Poly1305 · Double Ratchet · {activePeer?.latency_ms != null ? `${activePeer.latency_ms}ms` : 'mesh'}</div>
          </div>
          {activePeer && <div className={`ops-tag ops-tag-${activePeer.status==='online'?'sig':'muted'}`}>{activePeer.status}</div>}
        </div>

        <div className="msg-list">
          {loading ? (
            <div style={{ textAlign:'center',padding:24,fontFamily:'var(--mono)',fontSize:11,color:'var(--t4)' }}>Loading...</div>
          ) : localMsgs.length === 0 ? (
            <div style={{ textAlign:'center',padding:40,fontFamily:'var(--mono)',fontSize:11,color:'var(--t4)' }}>
              <div style={{ fontSize:28,marginBottom:8,color:'var(--border-hi)' }}>⟁</div>
              No messages yet
            </div>
          ) : localMsgs.map((m, i) => {
            const isMe = m.sender === myId || m.sender === state.profile?.username;
            return (
              <div key={m.id??i} style={{ display:'flex',flexDirection:'column',alignItems:isMe?'flex-end':'flex-start' }}>
                <div className={isMe?'msg-out':'msg-in'}>
                  <div style={{ wordBreak:'break-word' }}>{m.content}</div>
                  <div style={{ display:'flex',gap:6,alignItems:'center',marginTop:4,justifyContent:'flex-end' }}>
                    {m.encrypted && <span className="ops-tag ops-tag-sig" style={{ fontSize:8,padding:'1px 4px' }}>⬢ E2E</span>}
                    <span style={{ fontFamily:'var(--mono)',fontSize:9,color:'var(--t3)' }}>{m.timestamp?fmt(m.timestamp):''}</span>
                    {isMe && <span style={{ fontFamily:'var(--mono)',fontSize:9,color:'var(--t3)' }}>
                      {m.delivery_status === 'read' ? '✓✓' : m.delivery_status === 'delivered' ? '✓✓' : m.delivery_status === 'sent' ? '✓' : '○'}
                    </span>}
                  </div>
                </div>
                {!isMe && <div style={{ fontFamily:'var(--mono)',fontSize:9,color:'var(--t4)',margin:'2px 4px' }}>{(() => { const p = state.peers.find(x => x.id === m.sender || x.node_id === m.sender); return p?.nickname || p?.username || m.sender?.slice(0,10); })()}</div>}
              </div>
            );
          })}
          <div ref={endRef} />
        </div>

        <div style={{ padding:'12px 16px',borderTop:'1px solid var(--border)',background:'var(--surface)',flexShrink:0 }}>
          <div style={{ display:'flex',gap:8 }}>
            <textarea className="ops-input" rows={2}
              placeholder={recipient==='broadcast'?'Broadcast to all peers...': `Message ${activeName}...`}
              value={input} onChange={e => setInput(e.target.value)} onKeyDown={handleKey} disabled={sending}
              style={{ flex:1,resize:'none',lineHeight:1.4,fontFamily:'var(--mono)',fontSize:12 }}
            />
            <button className="btn-ops btn-ops-primary" onClick={handleSend} disabled={sending||!input.trim()}
              style={{ padding:'8px 14px',flexShrink:0,alignSelf:'flex-end',height:36 }}>
              {sending?'⟳':'⟁ Send'}
            </button>
          </div>
          <div style={{ fontFamily:'var(--mono)',fontSize:9,color:'var(--t4)',marginTop:5 }}>
            Enter to send · Shift+Enter newline · End-to-end encrypted
          </div>
          {deliveryState !== 'idle' && (
            <div className={`ops-tag ${deliveryState === 'failed' ? 'ops-tag-danger' : deliveryState === 'accepted' ? 'ops-tag-sig' : 'ops-tag-muted'}`} style={{ marginTop: 8 }}>
              {deliveryState === 'sending' && 'sending — awaiting daemon acceptance'}
              {deliveryState === 'accepted' && 'accepted by daemon — delivery receipt pending'}
              {deliveryState === 'failed' && `failed — ${deliveryError ?? 'operation rejected'}`}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
