import React, { useState, useEffect } from 'react';

const BOOT_LINES = [
  { text: 'GHOSTWIRE NODE INIT v2.0',              ok: false },
  { text: 'Loading cryptographic modules...',      ok: true  },
  { text: 'Mounting identity engine...',           ok: true  },
  { text: 'Verifying genesis constants [848CF75A]', ok: true },
  { text: 'Initialising Sphinx routing...',        ok: true  },
  { text: 'Starting vodozemac ratchet...',         ok: true  },
  { text: 'Scanning local transports (TCP/BT/LoRa)...', ok: true },
  { text: 'Configuring mDNS discovery...',         ok: true  },
  { text: 'Bootstrapping S/Kademlia DHT...',       ok: true  },
  { text: 'Starting WebSocket event bus...',       ok: true  },
  { text: 'NODE ONLINE.',                          ok: true  },
];

export default function LoadingScreen({ onComplete }: { onComplete: () => void }) {
  const [lines, setLines]       = useState<typeof BOOT_LINES>([]);
  const [progress, setProgress] = useState(0);
  const [blink, setBlink]       = useState(true);

  useEffect(() => {
    let i = 0;
    const tick = setInterval(() => {
      if (i < BOOT_LINES.length) {
        setLines(prev => [...prev, BOOT_LINES[i]]);
        setProgress(((i + 1) / BOOT_LINES.length) * 100);
        i++;
      } else {
        clearInterval(tick);
        setTimeout(onComplete, 600);
      }
    }, 280);
    return () => clearInterval(tick);
  }, [onComplete]);

  useEffect(() => {
    const t = setInterval(() => setBlink(b => !b), 500);
    return () => clearInterval(t);
  }, []);

  return (
    <div style={{
      position:'fixed', inset:0, zIndex:9999,
      background:'var(--void)',
      display:'flex', flexDirection:'column', alignItems:'center', justifyContent:'center',
      fontFamily:'var(--mono)',
    }}>
      {/* Corner decorations */}
      {[{top:20,left:20},{top:20,right:20},{bottom:20,left:20},{bottom:20,right:20}].map((pos,i) => (
        <div key={i} style={{
          position:'absolute', ...pos,
          width:20, height:20,
          borderTop: i<2 ? '1px solid var(--sig-dim)' : 'none',
          borderBottom: i>=2 ? '1px solid var(--sig-dim)' : 'none',
          borderLeft: i%2===0 ? '1px solid var(--sig-dim)' : 'none',
          borderRight: i%2===1 ? '1px solid var(--sig-dim)' : 'none',
          opacity:0.5,
        }} />
      ))}

      {/* Wordmark */}
      <div style={{ marginBottom:32, textAlign:'center' }}>
        <div style={{ fontSize:24, fontWeight:700, letterSpacing:'0.25em', color:'var(--sig)' }}>
          GHOSTWIRE
        </div>
        <div style={{ fontSize:9, letterSpacing:'0.2em', color:'var(--t4)', marginTop:3 }}>
          SECURE MESH COMMUNICATION
        </div>
      </div>

      {/* Terminal */}
      <div style={{
        width:'min(480px, 90vw)',
        background:'var(--panel)',
        border:'1px solid var(--border-hi)',
        borderRadius:'var(--r-md)',
        overflow:'hidden',
      }}>
        {/* Terminal header bar */}
        <div style={{
          padding:'6px 12px', background:'var(--raised)',
          borderBottom:'1px solid var(--border)',
          display:'flex', alignItems:'center', gap:6,
        }}>
          <div style={{ width:6,height:6,borderRadius:'50%',background:'var(--crit)' }} />
          <div style={{ width:6,height:6,borderRadius:'50%',background:'var(--warn)' }} />
          <div style={{ width:6,height:6,borderRadius:'50%',background:'var(--sig)' }} />
          <span style={{ marginLeft:8, fontSize:9, color:'var(--t4)', letterSpacing:'0.1em' }}>
            ghostwire — boot sequence
          </span>
        </div>

        {/* Progress bar */}
        <div style={{ height:2, background:'var(--border)' }}>
          <div style={{
            height:'100%', background:'var(--sig)',
            width:`${progress}%`,
            transition:'width 0.25s ease',
            boxShadow:'0 0 8px rgba(46,255,122,0.4)',
          }} />
        </div>

        {/* Log output */}
        <div style={{ padding:'12px 14px', minHeight:200, maxHeight:260, overflowY:'auto' }}>
          {lines.map((line, i) => (
            <div key={i} style={{
              display:'flex', gap:8, marginBottom:4, fontSize:11,
              animation:'fade-in 0.15s ease-out',
            }}>
              <span style={{ color:'var(--t4)', flexShrink:0, fontSize:10 }}>
                [{new Date().toLocaleTimeString('en',{hour12:false})}]
              </span>
              <span style={{
                color: i === lines.length-1 && line.text.includes('ONLINE') ? 'var(--sig)'
                     : line.ok ? 'var(--t2)' : 'var(--t1)',
                fontWeight: line.text.includes('ONLINE') ? 700 : 400,
              }}>
                {line.text}
                {line.ok && i < lines.length - 1 && (
                  <span style={{ color:'var(--sig)', marginLeft:6 }}>OK</span>
                )}
              </span>
            </div>
          ))}
          {lines.length > 0 && lines.length < BOOT_LINES.length && (
            <div style={{ display:'flex', gap:8, fontSize:11 }}>
              <span style={{ color:'var(--t4)', fontSize:10 }}>[{new Date().toLocaleTimeString('en',{hour12:false})}]</span>
              <span style={{ display:'inline-block', width:8, height:14, background:blink?'var(--sig)':'transparent', verticalAlign:'middle' }} />
            </div>
          )}
        </div>

        {/* Footer */}
        <div style={{
          padding:'6px 14px 8px',
          borderTop:'1px solid var(--border)',
          display:'flex', justifyContent:'space-between', alignItems:'center',
        }}>
          <span style={{ fontSize:9, color:'var(--t4)', letterSpacing:'0.1em' }}>NET·FP 848CF75A</span>
          <span style={{ fontSize:10, color:'var(--sig)', fontWeight:700 }}>{Math.floor(progress)}%</span>
        </div>
      </div>
    </div>
  );
}
