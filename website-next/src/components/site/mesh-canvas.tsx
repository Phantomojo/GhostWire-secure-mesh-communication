'use client'

import { useEffect, useRef } from "react";

interface Node {
  x: number;
  y: number;
  vx: number;
  vy: number;
  r: number;
}

interface Packet {
  from: number;
  to: number;
  t: number; // 0..1 progress
  speed: number;
}

export function MeshCanvas({ className = "" }: { className?: string }) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const prefersReduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    let width = 0;
    let height = 0;
    let dpr = 1;
    let nodes: Node[] = [];
    let packets: Packet[] = [];
    let rafId = 0;

    const MAX_NODES = 38;

    function resize() {
      if (!canvas || !ctx) return;
      const rect = canvas.getBoundingClientRect();
      dpr = Math.min(window.devicePixelRatio || 1, 2);
      width = rect.width;
      height = rect.height;
      canvas.width = Math.max(1, Math.floor(width * dpr));
      canvas.height = Math.max(1, Math.floor(height * dpr));
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      seed();
    }

    function seed() {
      nodes = [];
      const count = Math.min(MAX_NODES, Math.max(18, Math.floor((width * height) / 38000)));
      for (let i = 0; i < count; i++) {
        nodes.push({
          x: Math.random() * width,
          y: Math.random() * height,
          vx: (Math.random() - 0.5) * 0.18,
          vy: (Math.random() - 0.5) * 0.18,
          r: Math.random() * 1.6 + 1.0,
        });
      }
      packets = [];
      // seed a few packets
      for (let i = 0; i < 4; i++) spawnPacket();
    }

    function spawnPacket() {
      if (nodes.length < 2) return;
      const from = Math.floor(Math.random() * nodes.length);
      let to = Math.floor(Math.random() * nodes.length);
      while (to === from) to = Math.floor(Math.random() * nodes.length);
      packets.push({ from, to, t: 0, speed: 0.006 + Math.random() * 0.012 });
    }

    function step() {
      if (!ctx) return;
      ctx.clearRect(0, 0, width, height);

      // Update + draw nodes
      for (const n of nodes) {
        n.x += n.vx;
        n.y += n.vy;
        if (n.x < 0 || n.x > width) n.vx *= -1;
        if (n.y < 0 || n.y > height) n.vy *= -1;
        n.x = Math.max(0, Math.min(width, n.x));
        n.y = Math.max(0, Math.min(height, n.y));
      }

      // Draw edges between nearby nodes
      const maxDist = Math.min(width, height) * 0.22;
      for (let i = 0; i < nodes.length; i++) {
        for (let j = i + 1; j < nodes.length; j++) {
          const dx = nodes[i].x - nodes[j].x;
          const dy = nodes[i].y - nodes[j].y;
          const d = Math.hypot(dx, dy);
          if (d < maxDist) {
            const alpha = (1 - d / maxDist) * 0.22;
            ctx.strokeStyle = `rgba(0, 220, 154, ${alpha})`;
            ctx.lineWidth = 0.7;
            ctx.beginPath();
            ctx.moveTo(nodes[i].x, nodes[i].y);
            ctx.lineTo(nodes[j].x, nodes[j].y);
            ctx.stroke();
          }
        }
      }

      // Draw nodes (with glow on a few "hub" nodes)
      nodes.forEach((n, idx) => {
        const isHub = idx % 5 === 0;
        if (isHub) {
          const g = ctx.createRadialGradient(n.x, n.y, 0, n.x, n.y, n.r * 6);
          g.addColorStop(0, "rgba(0, 220, 154, 0.45)");
          g.addColorStop(1, "rgba(0, 220, 154, 0)");
          ctx.fillStyle = g;
          ctx.beginPath();
          ctx.arc(n.x, n.y, n.r * 6, 0, Math.PI * 2);
          ctx.fill();
        }
        ctx.fillStyle = isHub ? "#00DC9A" : "rgba(0, 220, 154, 0.55)";
        ctx.beginPath();
        ctx.arc(n.x, n.y, n.r, 0, Math.PI * 2);
        ctx.fill();
      });

      // Draw packets (messages hopping along the mesh)
      for (let i = packets.length - 1; i >= 0; i--) {
        const p = packets[i];
        const a = nodes[p.from];
        const b = nodes[p.to];
        if (!a || !b) {
          packets.splice(i, 1);
          continue;
        }
        p.t += p.speed;
        if (p.t >= 1) {
          packets.splice(i, 1);
          continue;
        }
        const x = a.x + (b.x - a.x) * p.t;
        const y = a.y + (b.y - a.y) * p.t;
        const g = ctx.createRadialGradient(x, y, 0, x, y, 8);
        g.addColorStop(0, "rgba(0, 220, 154, 0.9)");
        g.addColorStop(1, "rgba(0, 220, 154, 0)");
        ctx.fillStyle = g;
        ctx.beginPath();
        ctx.arc(x, y, 8, 0, Math.PI * 2);
        ctx.fill();
        ctx.fillStyle = "#F0EDE6";
        ctx.beginPath();
        ctx.arc(x, y, 1.6, 0, Math.PI * 2);
        ctx.fill();
      }

      // Occasionally spawn packets
      if (packets.length < 6 && Math.random() < 0.04) spawnPacket();

      rafId = requestAnimationFrame(step);
    }

    resize();
    const ro = new ResizeObserver(resize);
    ro.observe(canvas);

    if (!prefersReduced) {
      rafId = requestAnimationFrame(step);
    } else {
      // Draw a single static frame
      step();
      cancelAnimationFrame(rafId);
    }

    return () => {
      cancelAnimationFrame(rafId);
      ro.disconnect();
    };
  }, []);

  return (
    <canvas
      ref={canvasRef}
      aria-hidden="true"
      className={`absolute inset-0 w-full h-full pointer-events-none ${className}`}
      style={{ opacity: 0.5 }}
    />
  );
}
