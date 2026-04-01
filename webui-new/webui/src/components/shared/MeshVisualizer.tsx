import React, { useEffect, useRef, useCallback } from 'react';

interface MeshNode {
  id: string;
  name: string;
  status: string;
  trust_score?: number;
  latency_ms?: number;
  x?: number;
  y?: number;
  vx?: number;
  vy?: number;
}

interface MeshVisualizerProps {
  nodes: MeshNode[];
  localNickname?: string;
  onNodeClick?: (id: string) => void;
}

const SIG   = '#2EFF7A';
const WARN  = '#FFB800';
const CRIT  = '#FF3B5C';
const DIM   = 'rgba(46,255,122,0.15)';
const VOID  = '#05070A';

export default function MeshVisualizer({ nodes, localNickname = 'THIS NODE', onNodeClick }: MeshVisualizerProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const stateRef  = useRef<MeshNode[]>([]);
  const rafRef    = useRef<number>();
  const hoverRef  = useRef<string | null>(null);
  const selectedRef = useRef<string | null>(null);

  // Sync incoming nodes, preserve physics state
  useEffect(() => {
    const prev = stateRef.current;
    stateRef.current = nodes.map(n => {
      const old = prev.find(p => p.id === n.id);
      if (old) return { ...old, status: n.status, name: n.name, trust_score: n.trust_score, latency_ms: n.latency_ms };
      const angle = Math.random() * Math.PI * 2;
      const dist  = 80 + Math.random() * 140;
      const cx = canvasRef.current ? canvasRef.current.width / 2 : 400;
      const cy = canvasRef.current ? canvasRef.current.height / 2 : 250;
      return { ...n, x: cx + Math.cos(angle) * dist, y: cy + Math.sin(angle) * dist, vx: (Math.random()-.5)*0.4, vy: (Math.random()-.5)*0.4 };
    });
  }, [nodes]);

  // Mouse handlers
  const getNodeAt = useCallback((mx: number, my: number): string | null => {
    for (const n of stateRef.current) {
      if (!n.x || !n.y) continue;
      const d = Math.hypot(n.x - mx, n.y - my);
      if (d < 14) return n.id;
    }
    return null;
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const toCanvas = (e: MouseEvent) => {
      const r = canvas.getBoundingClientRect();
      return {
        x: (e.clientX - r.left) * (canvas.width / r.width),
        y: (e.clientY - r.top)  * (canvas.height / r.height),
      };
    };

    const onMove = (e: MouseEvent) => {
      const { x, y } = toCanvas(e);
      hoverRef.current = getNodeAt(x, y);
      canvas.style.cursor = hoverRef.current ? 'pointer' : 'default';
    };
    const onClick = (e: MouseEvent) => {
      const { x, y } = toCanvas(e);
      const id = getNodeAt(x, y);
      selectedRef.current = id;
      if (id && onNodeClick) onNodeClick(id);
    };
    canvas.addEventListener('mousemove', onMove);
    canvas.addEventListener('click', onClick);
    return () => { canvas.removeEventListener('mousemove', onMove); canvas.removeEventListener('click', onClick); };
  }, [getNodeAt, onNodeClick]);

  // Animation loop
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d')!;

    // Resize canvas to container
    const resize = () => {
      const rect = canvas.parentElement!.getBoundingClientRect();
      canvas.width  = rect.width  || 600;
      canvas.height = rect.height || 360;
    };
    resize();
    const ro = new ResizeObserver(resize);
    ro.observe(canvas.parentElement!);

    const draw = () => {
      const W = canvas.width, H = canvas.height;
      const cx = W / 2, cy = H / 2;
      const t = Date.now();

      // Clear with trail
      ctx.fillStyle = 'rgba(5,7,10,0.25)';
      ctx.fillRect(0, 0, W, H);

      // Grid dots
      ctx.fillStyle = 'rgba(46,255,122,0.04)';
      for (let x = 0; x < W; x += 36) for (let y = 0; y < H; y += 36) {
        ctx.beginPath(); ctx.arc(x, y, 0.6, 0, Math.PI*2); ctx.fill();
      }

      // Range rings
      ctx.setLineDash([2, 8]);
      ctx.lineWidth = 0.5;
      [80, 160, 240].forEach(r => {
        ctx.strokeStyle = `rgba(46,255,122,${r===80?0.08:r===160?0.05:0.03})`;
        ctx.beginPath(); ctx.arc(cx, cy, r, 0, Math.PI*2); ctx.stroke();
      });
      ctx.setLineDash([]);

      // Radar sweep
      const sweep = (t / 3000) % (Math.PI * 2);
      const sg = ctx.createConicalGradient ? undefined : null; // fallback
      ctx.save();
      ctx.translate(cx, cy);
      ctx.rotate(sweep);
      const rg = ctx.createLinearGradient(0, 0, 240, 0);
      rg.addColorStop(0, 'rgba(46,255,122,0.08)');
      rg.addColorStop(1, 'rgba(46,255,122,0)');
      ctx.fillStyle = rg;
      ctx.beginPath();
      ctx.moveTo(0, 0);
      ctx.arc(0, 0, 240, 0, 0.5);
      ctx.closePath();
      ctx.fill();
      ctx.restore();

      // Update physics + draw connections
      stateRef.current.forEach(n => {
        if (n.x == null || n.y == null) return;
        n.x! += n.vx!; n.y! += n.vy!;
        const dx = n.x! - cx, dy = n.y! - cy, dist = Math.hypot(dx, dy);
        if (dist > 220) { n.vx! -= dx * 0.0003; n.vy! -= dy * 0.0003; }
        // Damping
        n.vx! *= 0.999; n.vy! *= 0.999;

        const isOnline = n.status === 'online';
        const col = isOnline ? SIG : (n.trust_score && n.trust_score < 50) ? CRIT : WARN;
        const hover = hoverRef.current === n.id;
        const sel   = selectedRef.current === n.id;
        const alpha = Math.max(0.06, 0.5 - dist/600);

        // Connection line
        ctx.strokeStyle = `rgba(46,255,122,${alpha * (hover||sel ? 0.8 : 0.25)})`;
        ctx.lineWidth = hover||sel ? 1.5 : 0.8;
        ctx.beginPath(); ctx.moveTo(cx, cy); ctx.lineTo(n.x!, n.y!); ctx.stroke();

        // Signal flow dot
        if (isOnline) {
          const frac = ((t/1200) + n.id.charCodeAt(0)*0.17) % 1;
          ctx.fillStyle = SIG;
          ctx.beginPath();
          ctx.arc(cx + dx*frac, cy + dy*frac, 1.5, 0, Math.PI*2);
          ctx.fill();
        }
      });

      // Draw peer nodes
      stateRef.current.forEach(n => {
        if (n.x == null || n.y == null) return;
        const isOnline = n.status === 'online';
        const col = isOnline ? SIG : (n.trust_score && n.trust_score < 50) ? CRIT : WARN;
        const hover = hoverRef.current === n.id;
        const sel   = selectedRef.current === n.id;
        const r = hover ? 7 : sel ? 6 : 4.5;

        // Glow
        ctx.shadowBlur = hover ? 20 : 10;
        ctx.shadowColor = col;
        ctx.fillStyle = col;
        ctx.beginPath(); ctx.arc(n.x!, n.y!, r, 0, Math.PI*2); ctx.fill();
        ctx.shadowBlur = 0;

        // Selection ring
        if (sel) {
          ctx.strokeStyle = col;
          ctx.lineWidth = 1;
          ctx.setLineDash([3, 3]);
          ctx.beginPath(); ctx.arc(n.x!, n.y!, r+5, 0, Math.PI*2); ctx.stroke();
          ctx.setLineDash([]);
        }

        // Label
        ctx.fillStyle = hover ? '#E8EDF2' : 'rgba(232,237,242,0.6)';
        ctx.font = `${hover?'600':'400'} ${hover?10:9}px "JetBrains Mono", monospace`;
        ctx.textAlign = 'left';
        ctx.textBaseline = 'middle';
        ctx.fillText((n.name||n.id.slice(0,8)).toUpperCase(), n.x!+12, n.y!);

        if (hover) {
          ctx.fillStyle = 'rgba(46,255,122,0.5)';
          ctx.font = '8px "JetBrains Mono", monospace';
          ctx.fillText(`${n.status.toUpperCase()}${n.latency_ms!=null?` · ${n.latency_ms}ms`:''}`, n.x!+12, n.y!+12);
        }
      });

      // Local node (this machine)
      ctx.shadowBlur = 24;
      ctx.shadowColor = SIG;
      ctx.fillStyle = SIG;
      ctx.beginPath(); ctx.arc(cx, cy, 8, 0, Math.PI*2); ctx.fill();

      // Outer ring pulse
      const pulse = 0.4 + Math.sin(t/600)*0.2;
      ctx.strokeStyle = `rgba(46,255,122,${pulse})`;
      ctx.lineWidth = 1.5;
      ctx.beginPath(); ctx.arc(cx, cy, 14, 0, Math.PI*2); ctx.stroke();
      ctx.shadowBlur = 0;

      // Local node label
      ctx.fillStyle = '#E8EDF2';
      ctx.font = 'bold 9px "JetBrains Mono", monospace';
      ctx.textAlign = 'center';
      ctx.fillText(localNickname.toUpperCase(), cx, cy - 22);

      // HUD overlay
      ctx.fillStyle = 'rgba(46,255,122,0.6)';
      ctx.font = '700 9px "JetBrains Mono", monospace';
      ctx.textAlign = 'left';
      ctx.textBaseline = 'top';
      ctx.fillText(`PEERS: ${stateRef.current.length}`, 12, 12);
      ctx.fillText(`MESH: ${stateRef.current.filter(n=>n.status==='online').length} ACTIVE`, 12, 24);

      ctx.fillStyle = 'rgba(46,255,122,0.3)';
      ctx.fillText(new Date().toLocaleTimeString(), W - 70, 12);

      rafRef.current = requestAnimationFrame(draw);
    };

    rafRef.current = requestAnimationFrame(draw);
    return () => { ro.disconnect(); cancelAnimationFrame(rafRef.current!); };
  }, [localNickname]);

  return (
    <div style={{ width:'100%', height:'100%', background: VOID, position:'relative', overflow:'hidden' }}>
      <canvas ref={canvasRef} style={{ display:'block', width:'100%', height:'100%' }} />
      {nodes.length === 0 && (
        <div style={{
          position:'absolute', inset:0, display:'flex', alignItems:'center', justifyContent:'center',
          flexDirection:'column', gap:8, pointerEvents:'none',
        }}>
          <div style={{ fontFamily:'var(--mono)', fontSize:11, color:'var(--t4)' }}>No peers in mesh</div>
          <div style={{ fontFamily:'var(--mono)', fontSize:9, color:'var(--t4)' }}>mDNS scanning…</div>
        </div>
      )}
    </div>
  );
}
