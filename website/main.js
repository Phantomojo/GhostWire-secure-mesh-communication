/* ================================================
   GHOSTWIRE — MAIN JAVASCRIPT
   ================================================ */

// ---- REDUCED MOTION CHECK ----
const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

// ---- NAV SCROLL ----
const nav = document.getElementById('nav');
if (nav) {
  window.addEventListener('scroll', () => {
    nav.classList.toggle('scrolled', window.scrollY > 20);
  });
}

// ---- MOBILE NAV ----
const navToggle = document.getElementById('navToggle');
const navMobile = document.getElementById('navMobile');
if (navToggle && navMobile) {
  navToggle.addEventListener('click', () => {
    const isOpen = navMobile.classList.toggle('open');
    navToggle.setAttribute('aria-expanded', String(isOpen));
  });
}

// ---- HERO MESH CANVAS ----
const meshCanvas = document.getElementById('meshCanvas');
if (meshCanvas && !prefersReducedMotion) {
  const ctx = meshCanvas.getContext('2d');
  let nodes = [];
  let animFrame;

  const GREEN = '#00DC9A';
  const GREEN_DIM = 'rgba(0, 220, 154, ';

  function resize() {
    meshCanvas.width = meshCanvas.offsetWidth;
    meshCanvas.height = meshCanvas.offsetHeight;
    initNodes();
  }

  function initNodes() {
    nodes = [];
    const count = Math.floor((meshCanvas.width * meshCanvas.height) / 18000);
    for (let i = 0; i < count; i++) {
      nodes.push({
        x: Math.random() * meshCanvas.width,
        y: Math.random() * meshCanvas.height,
        vx: (Math.random() - 0.5) * 0.35,
        vy: (Math.random() - 0.5) * 0.35,
        r: Math.random() * 1.5 + 0.5,
        pulse: Math.random() * Math.PI * 2,
        pulseSpeed: 0.01 + Math.random() * 0.015,
        active: Math.random() > 0.65
      });
    }
  }

  function drawMesh() {
    ctx.clearRect(0, 0, meshCanvas.width, meshCanvas.height);

    const MAX_DIST = 180;

    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const dx = nodes[i].x - nodes[j].x;
        const dy = nodes[i].y - nodes[j].y;
        const d = Math.sqrt(dx * dx + dy * dy);
        if (d < MAX_DIST) {
          const alpha = (1 - d / MAX_DIST) * 0.25;
          ctx.beginPath();
          ctx.moveTo(nodes[i].x, nodes[i].y);
          ctx.lineTo(nodes[j].x, nodes[j].y);
          ctx.strokeStyle = GREEN_DIM + alpha + ')';
          ctx.lineWidth = 0.5;
          ctx.stroke();
        }
      }
    }

    for (let n of nodes) {
      n.pulse += n.pulseSpeed;
      const pv = (Math.sin(n.pulse) + 1) / 2;

      if (n.active) {
        ctx.beginPath();
        ctx.arc(n.x, n.y, n.r * 2.5 * pv + 2, 0, Math.PI * 2);
        ctx.fillStyle = GREEN_DIM + (0.05 * pv) + ')';
        ctx.fill();
      }

      ctx.beginPath();
      ctx.arc(n.x, n.y, n.r, 0, Math.PI * 2);
      ctx.fillStyle = n.active
        ? GREEN_DIM + (0.5 + pv * 0.5) + ')'
        : GREEN_DIM + '0.2)';
      ctx.fill();

      n.x += n.vx;
      n.y += n.vy;
      if (n.x < 0 || n.x > meshCanvas.width) n.vx *= -1;
      if (n.y < 0 || n.y > meshCanvas.height) n.vy *= -1;
    }

    animFrame = requestAnimationFrame(drawMesh);
  }

  const ro = new ResizeObserver(resize);
  ro.observe(meshCanvas.parentElement);
  resize();
  drawMesh();
} else if (meshCanvas && prefersReducedMotion) {
  meshCanvas.style.background = 'radial-gradient(ellipse at 30% 40%, rgba(0,220,154,0.06) 0%, transparent 60%)';
}

// ---- DIAGRAM CANVAS (How It Works) ----
const diagramCanvas = document.getElementById('diagramCanvas');
if (diagramCanvas && !prefersReducedMotion) {
  const dctx = diagramCanvas.getContext('2d');
  const GREEN = '#00DC9A';

  const meshNodes = [
    { x: 0.5, y: 0.25, label: 'Node A', active: true },
    { x: 0.15, y: 0.55, label: 'Node B', active: true },
    { x: 0.75, y: 0.55, label: 'Node C', active: false },
    { x: 0.35, y: 0.8, label: 'Node D', active: true },
    { x: 0.65, y: 0.82, label: 'Node E', active: true },
    { x: 0.88, y: 0.3, label: 'Node F', active: false },
  ];

  const edges = [[0,1],[0,2],[0,5],[1,3],[2,3],[2,4],[2,5],[3,4],[4,5]];

  let dAngle = 0;
  let packet = { edge: 0, t: 0, speed: 0.008 };

  function drawDiagram() {
    const W = diagramCanvas.width;
    const H = diagramCanvas.height;
    dctx.clearRect(0, 0, W, H);

    dAngle += 0.003;

    for (let [a, b] of edges) {
      const na = meshNodes[a], nb = meshNodes[b];
      const active = na.active && nb.active;
      dctx.beginPath();
      dctx.moveTo(na.x * W, na.y * H);
      dctx.lineTo(nb.x * W, nb.y * H);
      dctx.strokeStyle = active ? 'rgba(0,220,154,0.22)' : 'rgba(0,220,154,0.07)';
      dctx.lineWidth = 1;
      dctx.stroke();
    }

    for (let n of meshNodes) {
      const x = n.x * W, y = n.y * H;
      if (n.active) {
        dctx.beginPath();
        dctx.arc(x, y, 18 + Math.sin(dAngle * 3 + n.x * 10) * 4, 0, Math.PI * 2);
        dctx.fillStyle = 'rgba(0,220,154,0.05)';
        dctx.fill();
      }
      dctx.beginPath();
      dctx.arc(x, y, n.active ? 7 : 5, 0, Math.PI * 2);
      dctx.fillStyle = n.active ? GREEN : 'rgba(0,220,154,0.25)';
      dctx.fill();

      dctx.font = '10px IBM Plex Mono, monospace';
      dctx.fillStyle = n.active ? 'rgba(0,220,154,0.7)' : 'rgba(0,220,154,0.25)';
      dctx.textAlign = 'center';
      dctx.fillText(n.label, x, y - 14);
    }

    packet.t += packet.speed;
    if (packet.t >= 1) {
      packet.t = 0;
      packet.edge = (packet.edge + 1) % edges.length;
    }

    const [ea, eb] = edges[packet.edge];
    const na = meshNodes[ea], nb = meshNodes[eb];
    const px = (na.x + (nb.x - na.x) * packet.t) * W;
    const py = (na.y + (nb.y - na.y) * packet.t) * H;
    dctx.beginPath();
    dctx.arc(px, py, 3.5, 0, Math.PI * 2);
    dctx.fillStyle = '#fff';
    dctx.fill();
    dctx.beginPath();
    dctx.arc(px, py, 7, 0, Math.PI * 2);
    dctx.fillStyle = 'rgba(255,255,255,0.1)';
    dctx.fill();

    requestAnimationFrame(drawDiagram);
  }

  function resizeDiagram() {
    const parent = diagramCanvas.parentElement;
    diagramCanvas.width = parent.offsetWidth;
    diagramCanvas.height = parent.offsetHeight;
  }

  new ResizeObserver(resizeDiagram).observe(diagramCanvas.parentElement);
  resizeDiagram();
  drawDiagram();
} else if (diagramCanvas && prefersReducedMotion) {
  const dctx = diagramCanvas.getContext('2d');
  const GREEN = '#00DC9A';

  const meshNodes = [
    { x: 0.5, y: 0.25, label: 'Node A', active: true },
    { x: 0.15, y: 0.55, label: 'Node B', active: true },
    { x: 0.75, y: 0.55, label: 'Node C', active: false },
    { x: 0.35, y: 0.8, label: 'Node D', active: true },
    { x: 0.65, y: 0.82, label: 'Node E', active: true },
    { x: 0.88, y: 0.3, label: 'Node F', active: false },
  ];

  const edges = [[0,1],[0,2],[0,5],[1,3],[2,3],[2,4],[2,5],[3,4],[4,5]];

  function drawStaticDiagram() {
    const W = diagramCanvas.width;
    const H = diagramCanvas.height;
    dctx.clearRect(0, 0, W, H);

    for (let [a, b] of edges) {
      const na = meshNodes[a], nb = meshNodes[b];
      const active = na.active && nb.active;
      dctx.beginPath();
      dctx.moveTo(na.x * W, na.y * H);
      dctx.lineTo(nb.x * W, nb.y * H);
      dctx.strokeStyle = active ? 'rgba(0,220,154,0.22)' : 'rgba(0,220,154,0.07)';
      dctx.lineWidth = 1;
      dctx.stroke();
    }

    for (let n of meshNodes) {
      const x = n.x * W, y = n.y * H;
      if (n.active) {
        dctx.beginPath();
        dctx.arc(x, y, 18, 0, Math.PI * 2);
        dctx.fillStyle = 'rgba(0,220,154,0.05)';
        dctx.fill();
      }
      dctx.beginPath();
      dctx.arc(x, y, n.active ? 7 : 5, 0, Math.PI * 2);
      dctx.fillStyle = n.active ? GREEN : 'rgba(0,220,154,0.25)';
      dctx.fill();

      dctx.font = '10px IBM Plex Mono, monospace';
      dctx.fillStyle = n.active ? 'rgba(0,220,154,0.7)' : 'rgba(0,220,154,0.25)';
      dctx.textAlign = 'center';
      dctx.fillText(n.label, x, y - 14);
    }
  }

  function resizeDiagram() {
    const parent = diagramCanvas.parentElement;
    diagramCanvas.width = parent.offsetWidth;
    diagramCanvas.height = parent.offsetHeight;
    drawStaticDiagram();
  }

  new ResizeObserver(resizeDiagram).observe(diagramCanvas.parentElement);
  resizeDiagram();
}

// ---- MODAL SYSTEM ----
function openModal(modalId) {
  const modal = document.getElementById(modalId);
  if (!modal) return;
  modal.classList.add('active');
  document.body.style.overflow = 'hidden';
  // Focus the close button for accessibility
  const closeBtn = modal.querySelector('.modal-close');
  if (closeBtn) closeBtn.focus();
  // Trap focus
  trapFocus(modal);
}

function closeModal(modal) {
  if (typeof modal === 'string') modal = document.getElementById(modal);
  if (!modal) return;
  modal.classList.remove('active');
  document.body.style.overflow = '';
}

function trapFocus(modal) {
  const focusable = modal.querySelectorAll(
    'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
  );
  const first = focusable[0];
  const last = focusable[focusable.length - 1];

  modal.addEventListener('keydown', function handler(e) {
    if (e.key === 'Escape') {
      closeModal(modal);
      return;
    }
    if (e.key === 'Tab') {
      if (e.shiftKey) {
        if (document.activeElement === first) {
          e.preventDefault();
          last.focus();
        }
      } else {
        if (document.activeElement === last) {
          e.preventDefault();
          first.focus();
        }
      }
    }
  });
}

// Click handlers for data-modal cards and features
document.querySelectorAll('[data-modal]').forEach(trigger => {
  trigger.addEventListener('click', () => {
    openModal(trigger.dataset.modal);
  });
  // Keyboard support for role="button" elements
  trigger.addEventListener('keydown', (e) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      openModal(trigger.dataset.modal);
    }
  });
});

// Close on overlay click
document.querySelectorAll('.modal-overlay').forEach(overlay => {
  overlay.addEventListener('click', (e) => {
    if (e.target === overlay) {
      closeModal(overlay);
    }
  });
});

// Close on Escape (global fallback)
document.addEventListener('keydown', (e) => {
  if (e.key === 'Escape') {
    document.querySelectorAll('.modal-overlay.active').forEach(m => closeModal(m));
  }
});

// ---- QUICK START TABS ----
const qsTabs = document.querySelectorAll('.qs-tab');
const qsPanels = document.querySelectorAll('.qs-panel');

function activateTab(tab) {
  qsTabs.forEach(t => {
    t.classList.remove('active');
    t.setAttribute('aria-selected', 'false');
    t.setAttribute('tabindex', '-1');
  });
  qsPanels.forEach(p => p.classList.remove('active'));
  tab.classList.add('active');
  tab.setAttribute('aria-selected', 'true');
  tab.setAttribute('tabindex', '0');
  const target = document.getElementById('tab-' + tab.dataset.tab);
  if (target) target.classList.add('active');
}

qsTabs.forEach(tab => {
  tab.addEventListener('click', () => activateTab(tab));

  tab.addEventListener('keydown', (e) => {
    const tabArray = Array.from(qsTabs);
    const currentIndex = tabArray.indexOf(tab);
    let newIndex;

    if (e.key === 'ArrowRight' || e.key === 'ArrowDown') {
      e.preventDefault();
      newIndex = (currentIndex + 1) % tabArray.length;
    } else if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') {
      e.preventDefault();
      newIndex = (currentIndex - 1 + tabArray.length) % tabArray.length;
    } else if (e.key === 'Home') {
      e.preventDefault();
      newIndex = 0;
    } else if (e.key === 'End') {
      e.preventDefault();
      newIndex = tabArray.length - 1;
    } else if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      activateTab(tab);
      return;
    }

    if (newIndex !== undefined) {
      tabArray[newIndex].focus();
      activateTab(tabArray[newIndex]);
    }
  });
});

// ---- SMOOTH SCROLL FOR ANCHOR LINKS ----
document.querySelectorAll('a[href^="#"]').forEach(a => {
  a.addEventListener('click', e => {
    const target = document.querySelector(a.getAttribute('href'));
    if (target) {
      e.preventDefault();
      target.scrollIntoView({ behavior: prefersReducedMotion ? 'auto' : 'smooth', block: 'start' });
    }
  });
});

// ---- INTERSECTION OBSERVER — FADE IN ----
const fadeEls = document.querySelectorAll('.persona-card, .feature, .hiw-step, .community-card, .limitation');
const observer = new IntersectionObserver((entries) => {
  entries.forEach(entry => {
    if (entry.isIntersecting) {
      entry.target.style.opacity = '1';
      entry.target.style.transform = 'translateY(0)';
    }
  });
}, { threshold: 0.1, rootMargin: '0px 0px -40px 0px' });

fadeEls.forEach((el, i) => {
  if (prefersReducedMotion) {
    el.style.opacity = '1';
    el.style.transform = 'none';
  } else {
    el.style.opacity = '0';
    el.style.transform = 'translateY(16px)';
    el.style.transition = `opacity 0.5s ease ${i * 0.04}s, transform 0.5s ease ${i * 0.04}s`;
    observer.observe(el);
  }
});
