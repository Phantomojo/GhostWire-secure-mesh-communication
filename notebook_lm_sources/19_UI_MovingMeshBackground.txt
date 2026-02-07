import { useEffect, useRef } from 'react';

const MovingMeshBackground = ({ severity = 0.05 }: { severity?: number }) => {
    const canvasRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        const canvas = canvasRef.current;
        if (!canvas) return;

        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        let width = window.innerWidth;
        let height = window.innerHeight;
        canvas.width = width;
        canvas.height = height;

        // Configuration - Adjusted by severity
        const particleCount = Math.floor(Math.min(180, ((width * height) / 9000) * (0.8 + severity * 2)));
        const connectionDistance = 160 + severity * 100;
        const moveSpeed = 0.6 + severity * 1.5;
        const mouseRadius = 200;

        const particleColor = severity > 0.1 ? `rgba(168, 85, 247, ${0.2 + severity * 0.2})` : 'rgba(139, 92, 246, 0.25)';
        const lineColor = severity > 0.1 ? `rgba(139, 92, 246, ` : `rgba(168, 85, 247, `;

        interface Particle {
            x: number;
            y: number;
            vx: number;
            vy: number;
            size: number;
            originX: number;
            originY: number;
        }

        const particles: Particle[] = [];
        const mouse = { x: -1000, y: -1000 };

        // Initialize particles
        for (let i = 0; i < particleCount; i++) {
            const x = Math.random() * width;
            const y = Math.random() * height;
            particles.push({
                x,
                y,
                originX: x,
                originY: y,
                vx: (Math.random() - 0.5) * moveSpeed,
                vy: (Math.random() - 0.5) * moveSpeed,
                size: Math.random() * 2 + 0.5,
            });
        }

        const animate = () => {
            ctx.clearRect(0, 0, width, height);

            // Update and draw particles
            particles.forEach((p, i) => {
                // Natural movement
                p.x += p.vx;
                p.y += p.vy;

                // Mouse interaction (Neural response)
                const dxm = mouse.x - p.x;
                const dym = mouse.y - p.y;
                const distMouse = Math.sqrt(dxm * dxm + dym * dym);

                if (distMouse < mouseRadius) {
                    const force = (mouseRadius - distMouse) / mouseRadius;
                    const angle = Math.atan2(dym, dxm);
                    // Gentle attraction to simulate "focus"
                    p.x += Math.cos(angle) * force * 2;
                    p.y += Math.sin(angle) * force * 2;
                }

                // Bounce off edges with damping
                if (p.x < 0 || p.x > width) p.vx *= -0.9;
                if (p.y < 0 || p.y > height) p.vy *= -0.9;

                // Draw particle pulse
                const pulse = Math.sin(Date.now() / 1000 + i) * 0.5 + 1;
                ctx.beginPath();
                ctx.arc(p.x, p.y, p.size * pulse, 0, Math.PI * 2);
                ctx.fillStyle = particleColor;
                ctx.fill();

                // Connect particles
                for (let j = i + 1; j < particles.length; j++) {
                    const p2 = particles[j];
                    const dx = p.x - p2.x;
                    const dy = p.y - p2.y;
                    const distance = Math.sqrt(dx * dx + dy * dy);

                    if (distance < connectionDistance) {
                        ctx.beginPath();
                        const opacity = 0.2 * (1 - distance / connectionDistance);
                        ctx.strokeStyle = `${lineColor}${opacity})`;
                        // Thicker lines if near mouse
                        ctx.lineWidth = distMouse < mouseRadius * 0.8 ? 1.5 : 0.8;
                        ctx.moveTo(p.x, p.y);
                        ctx.lineTo(p2.x, p2.y);
                        ctx.stroke();
                    }
                }
            });

            requestAnimationFrame(animate);
        };

        const animationId = requestAnimationFrame(animate);

        const handleMouseMove = (e: MouseEvent) => {
            mouse.x = e.clientX;
            mouse.y = e.clientY;
        };

        const handleResize = () => {
            width = window.innerWidth;
            height = window.innerHeight;
            canvas.width = width;
            canvas.height = height;
        };

        window.addEventListener('mousemove', handleMouseMove);
        window.addEventListener('resize', handleResize);

        return () => {
            cancelAnimationFrame(animationId);
            window.removeEventListener('mousemove', handleMouseMove);
            window.removeEventListener('resize', handleResize);
        };
    }, [severity]);

    return (
        <canvas
            ref={canvasRef}
            className="fixed inset-0 pointer-events-none z-0"
            style={{ background: 'radial-gradient(ellipse at center, #0B1120 0%, #020617 100%)' }}
        />
    );
};

export default MovingMeshBackground;
