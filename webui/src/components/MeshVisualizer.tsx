import React, { useEffect, useRef } from 'react';

interface Node {
    id: string;
    name: string;
    x: number;
    y: number;
    vx: number;
    vy: number;
    status: 'online' | 'offline' | 'warning';
}

interface MeshVisualizerProps {
    nodes: { id: string; name: string; status: 'online' | 'offline' | 'warning' }[];
    onNodeClick?: (nodeId: string) => void;
}

const MeshVisualizer: React.FC<MeshVisualizerProps> = ({ nodes, onNodeClick }) => {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const nodesRef = useRef<Node[]>([]);
    const requestRef = useRef<number | null>(null);
    const [hoveredNodeId, setHoveredNodeId] = React.useState<string | null>(null);
    const [selectedNodeId, setSelectedNodeId] = React.useState<string | null>(null);

    useEffect(() => {
        // Maintain existing nodes positions/velocities if possible
        const currentNodes = nodesRef.current;
        nodesRef.current = nodes.map((node) => {
            const existing = currentNodes.find(n => n.id === node.id);
            if (existing) return { ...existing, status: node.status, name: node.name };

            const angle = Math.random() * Math.PI * 2;
            const dist = 100 + Math.random() * 150;
            return {
                ...node,
                x: 400 + Math.cos(angle) * dist,
                y: 250 + Math.sin(angle) * dist,
                vx: (Math.random() - 0.5) * 0.5,
                vy: (Math.random() - 0.5) * 0.5,
            };
        });

        const handleMouseMove = (e: MouseEvent) => {
            const canvas = canvasRef.current;
            if (!canvas) return;
            const rect = canvas.getBoundingClientRect();
            const mouseX = (e.clientX - rect.left) * (canvas.width / rect.width);
            const mouseY = (e.clientY - rect.top) * (canvas.height / rect.height);

            let found: string | null = null;
            nodesRef.current.forEach(node => {
                const dist = Math.sqrt((node.x - mouseX) ** 2 + (node.y - mouseY) ** 2);
                if (dist < 15) found = node.id;
            });
            setHoveredNodeId(found);
            canvas.style.cursor = found ? 'pointer' : 'default';
        };

        const handleClick = (e: MouseEvent) => {
            const canvas = canvasRef.current;
            if (!canvas) return;
            const rect = canvas.getBoundingClientRect();
            const mouseX = (e.clientX - rect.left) * (canvas.width / rect.width);
            const mouseY = (e.clientY - rect.top) * (canvas.height / rect.height);

            let clickedNodeId: string | null = null;
            nodesRef.current.forEach(node => {
                const dist = Math.sqrt((node.x - mouseX) ** 2 + (node.y - mouseY) ** 2);
                if (dist < 15) {
                    clickedNodeId = node.id;
                }
            });
            setSelectedNodeId(clickedNodeId);
            if (clickedNodeId && onNodeClick) {
                onNodeClick(clickedNodeId);
            }
        };

        const canvas = canvasRef.current;
        if (canvas) {
            canvas.addEventListener('mousemove', handleMouseMove);
            canvas.addEventListener('click', handleClick);
        }

        const animate = () => {
            const canvas = canvasRef.current;
            if (!canvas) return;
            const ctx = canvas.getContext('2d');
            if (!ctx) return;

            ctx.fillStyle = 'rgba(2, 6, 23, 0.2)';
            ctx.fillRect(0, 0, canvas.width, canvas.height);

            // Draw grid dots
            ctx.fillStyle = 'rgba(99, 102, 241, 0.05)';
            for (let x = 0; x < canvas.width; x += 40) {
                for (let y = 0; y < canvas.height; y += 40) {
                    ctx.beginPath();
                    ctx.arc(x, y, 0.5, 0, Math.PI * 2);
                    ctx.fill();
                }
            }

            const localX = canvas.width / 2;
            const localY = canvas.height / 2;

            // Draw range circles
            ctx.strokeStyle = 'rgba(99, 102, 241, 0.03)';
            ctx.lineWidth = 1;
            ctx.setLineDash([2, 5]);
            [100, 200, 300].forEach(radius => {
                ctx.beginPath();
                ctx.arc(localX, localY, radius, 0, Math.PI * 2);
                ctx.stroke();
            });
            ctx.setLineDash([]);

            // Update positions & draw connections
            nodesRef.current.forEach((node) => {
                node.x += node.vx;
                node.y += node.vy;

                const dx = node.x - localX;
                const dy = node.y - localY;
                const distToCenter = Math.sqrt(dx * dx + dy * dy);
                if (distToCenter > 350) {
                    node.vx -= dx * 0.0002;
                    node.vy -= dy * 0.0002;
                }

                const alpha = Math.max(0.05, 1 - distToCenter / 500);
                const isHovered = node.id === hoveredNodeId;
                const isSelected = node.id === selectedNodeId;

                ctx.strokeStyle = `rgba(99, 102, 241, ${alpha * (isHovered || isSelected ? 0.8 : 0.3)})`;
                ctx.lineWidth = isHovered || isSelected ? 2 : 1;
                ctx.beginPath();
                ctx.moveTo(localX, localY);
                ctx.lineTo(node.x, node.y);
                ctx.stroke();
            });

            // Draw nodes
            nodesRef.current.forEach((node) => {
                const isOnline = node.status === 'online';
                const isHovered = node.id === hoveredNodeId;
                const isSelected = node.id === selectedNodeId;
                const baseColor = isOnline ? '#6366f1' : node.status === 'warning' ? '#f59e0b' : '#ef4444';

                // Outer glow
                const pulse = Math.sin(Date.now() / 400) * 0.3 + 1;
                ctx.shadowBlur = (isHovered || isSelected ? 30 : 15) * pulse;
                ctx.shadowColor = baseColor;

                // Selection ring
                if (isSelected) {
                    ctx.strokeStyle = '#fff';
                    ctx.lineWidth = 2;
                    ctx.beginPath();
                    ctx.arc(node.x, node.y, 8, 0, Math.PI * 2);
                    ctx.stroke();
                }

                ctx.fillStyle = baseColor;
                ctx.beginPath();
                ctx.arc(node.x, node.y, isHovered ? 6 : 4, 0, Math.PI * 2);
                ctx.fill();

                ctx.shadowBlur = 0;
                ctx.fillStyle = '#fff';
                ctx.font = `bold ${isHovered ? '11px' : '9px'} "Inter", sans-serif`;
                ctx.textAlign = 'left';
                ctx.textBaseline = 'middle';
                ctx.fillText(node.name.toUpperCase(), node.x + 12, node.y);

                if (isHovered || isSelected) {
                    ctx.fillStyle = 'rgba(255, 255, 255, 0.5)';
                    ctx.font = '700 8px "Inter", sans-serif';
                    ctx.fillText(`${node.status.toUpperCase()} • CLICK TO ACTION`, node.x + 12, node.y + 12);
                }
            });

            // Draw local node
            ctx.shadowBlur = 25;
            ctx.shadowColor = '#6366f1';
            ctx.fillStyle = '#6366f1';
            ctx.beginPath();
            ctx.arc(localX, localY, 10, 0, Math.PI * 2);
            ctx.fill();

            ctx.shadowBlur = 0;
            ctx.strokeStyle = '#fff';
            ctx.lineWidth = 2;
            ctx.beginPath();
            ctx.arc(localX, localY, 15, 0, Math.PI * 2);
            ctx.stroke();

            ctx.fillStyle = '#fff';
            ctx.font = 'bold 10px "Inter", sans-serif';
            ctx.textAlign = 'center';
            ctx.fillText('COREHOST', localX, localY - 25);

            requestRef.current = requestAnimationFrame(animate);
        };

        requestRef.current = requestAnimationFrame(animate);
        return () => {
            if (canvas) {
                canvas.removeEventListener('mousemove', handleMouseMove);
                canvas.removeEventListener('click', handleClick);
            }
            if (requestRef.current) cancelAnimationFrame(requestRef.current);
        };
    }, [nodes, onNodeClick, hoveredNodeId, selectedNodeId]);

    return (
        <div className="relative w-full h-full flex items-center justify-center bg-slate-950/20">
            <canvas
                ref={canvasRef}
                width={800}
                height={500}
                className="max-w-full max-h-full object-contain"
            />
            {/* HUD Scan Line Overlay */}
            <div className="absolute inset-0 pointer-events-none overflow-hidden opacity-10">
                <div className="absolute w-full h-1 bg-indigo-500 shadow-[0_0_15px_indigo] animate-scan-y top-0" />
            </div>
            {/* Grid Overlay */}
            <div className="absolute inset-0 pointer-events-none bg-[radial-gradient(circle_at_center,transparent_0%,rgba(2,6,23,0.8)_100%)]" />
        </div>
    );
};

export default MeshVisualizer;
