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
}

const MeshVisualizer: React.FC<MeshVisualizerProps> = ({ nodes }) => {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const nodesRef = useRef<Node[]>([]);
    const requestRef = useRef<number | null>(null);

    useEffect(() => {
        // Initialize nodes with random positions and velocities
        nodesRef.current = nodes.map((node) => ({
            ...node,
            x: Math.random() * 600,
            y: Math.random() * 400,
            vx: (Math.random() - 0.5) * 2,
            vy: (Math.random() - 0.5) * 2,
        }));

        const animate = () => {
            const canvas = canvasRef.current;
            if (!canvas) return;
            const ctx = canvas.getContext('2d');
            if (!ctx) return;

            // Clear canvas
            ctx.clearRect(0, 0, canvas.width, canvas.height);

            // Update positions
            nodesRef.current.forEach((node) => {
                node.x += node.vx;
                node.y += node.vy;

                // Bounce off walls
                if (node.x < 0 || node.x > canvas.width) node.vx *= -1;
                if (node.y < 0 || node.y > canvas.height) node.vy *= -1;
            });

            // Draw connections
            ctx.strokeStyle = 'rgba(0, 255, 255, 0.1)';
            ctx.lineWidth = 1;
            for (let i = 0; i < nodesRef.current.length; i++) {
                for (let j = i + 1; j < nodesRef.current.length; j++) {
                    const n1 = nodesRef.current[i];
                    const n2 = nodesRef.current[j];
                    const dist = Math.sqrt((n1.x - n2.x) ** 2 + (n1.y - n2.y) ** 2);
                    if (dist < 150) {
                        ctx.beginPath();
                        ctx.moveTo(n1.x, n1.y);
                        ctx.lineTo(n2.x, n2.y);
                        ctx.stroke();
                    }
                }
            }

            // Draw local node connections
            const localX = canvas.width / 2;
            const localY = canvas.height / 2;
            ctx.strokeStyle = 'rgba(0, 150, 255, 0.3)';
            nodesRef.current.forEach((node) => {
                ctx.beginPath();
                ctx.moveTo(localX, localY);
                ctx.lineTo(node.x, node.y);
                ctx.stroke();
            });

            // Draw nodes
            nodesRef.current.forEach((node) => {
                const color = node.status === 'online' ? '#00ffcc' : node.status === 'warning' ? '#ffcc00' : '#ff3333';

                // Node pulse effect
                const pulse = Math.sin(Date.now() / 200) * 2 + 5;

                ctx.shadowBlur = pulse;
                ctx.shadowColor = color;
                ctx.fillStyle = color;
                ctx.beginPath();
                ctx.arc(node.x, node.y, 4, 0, Math.PI * 2);
                ctx.fill();

                ctx.shadowBlur = 0;
                ctx.fillStyle = 'rgba(255, 255, 255, 0.7)';
                ctx.font = '10px "Courier New"';
                ctx.fillText(node.name, node.x + 8, node.y + 3);
            });

            // Draw local node
            ctx.fillStyle = '#0066ff';
            ctx.shadowBlur = 10;
            ctx.shadowColor = '#0066ff';
            ctx.beginPath();
            ctx.arc(localX, localY, 8, 0, Math.PI * 2);
            ctx.fill();
            ctx.fillStyle = '#fff';
            ctx.font = 'bold 12px "Courier New"';
            ctx.textAlign = 'center';
            ctx.fillText('LOCAL', localX, localY - 15);

            requestRef.current = requestAnimationFrame(animate);
        };

        requestRef.current = requestAnimationFrame(animate);
        return () => cancelAnimationFrame(requestRef.current!);
    }, [nodes]);

    return (
        <div className="mesh-visualizer-container">
            <canvas
                ref={canvasRef}
                width={600}
                height={400}
                className="mesh-canvas"
            />
            <div className="canvas-overlay">
                <div className="grid-layer"></div>
                <div className="scan-line"></div>
            </div>
        </div>
    );
};

export default MeshVisualizer;
