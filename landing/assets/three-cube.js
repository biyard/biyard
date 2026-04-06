/**
 * Biyard Hyper Core — Three.js
 * Glass cube with face content, inner nucleus, hologram rings,
 * orbiting brand entities with pulsing energy lines, warp grid.
 * Mobile-responsive with touch support.
 */
(function () {
  "use strict";

  let scene, camera, renderer, coreGroup, movingGrid, particles;
  let animating = false;
  const orbitEntities = [];
  let mouseX = 0, mouseY = 0, targetX = 0, targetY = 0, scrollScale = 1;

  function makeFaceTexture(emoji, value, label, color) {
    var c = document.createElement("canvas");
    c.width = 512; c.height = 512;
    var ctx = c.getContext("2d");
    ctx.fillStyle = "rgba(5,10,20,0.92)";
    ctx.fillRect(0, 0, 512, 512);
    ctx.strokeStyle = color; ctx.lineWidth = 4;
    ctx.shadowColor = color; ctx.shadowBlur = 20;
    ctx.strokeRect(8, 8, 496, 496); ctx.shadowBlur = 0;
    ctx.font = "72px serif"; ctx.textAlign = "center";
    ctx.fillText(emoji, 256, 195);
    if (value) {
      ctx.font = "bold 44px sans-serif"; ctx.fillStyle = color;
      ctx.shadowColor = color; ctx.shadowBlur = 12;
      ctx.fillText(value, 256, 290); ctx.shadowBlur = 0;
    }
    ctx.font = "bold 26px sans-serif"; ctx.fillStyle = "rgba(255,255,255,0.45)";
    ctx.fillText(label.toUpperCase(), 256, 365);
    return new THREE.CanvasTexture(c);
  }

  function makeBrandTexture(name, color) {
    var c = document.createElement("canvas");
    c.width = 256; c.height = 256;
    var ctx = c.getContext("2d");
    ctx.fillStyle = "rgba(5,8,16,0.88)";
    ctx.fillRect(0, 0, 256, 256);
    ctx.strokeStyle = color; ctx.lineWidth = 3;
    ctx.shadowColor = color; ctx.shadowBlur = 10;
    ctx.strokeRect(4, 4, 248, 248); ctx.shadowBlur = 0;
    ctx.font = "bold 26px sans-serif"; ctx.fillStyle = color;
    ctx.textAlign = "center"; ctx.fillText(name, 128, 135);
    ctx.beginPath(); ctx.arc(128, 172, 5, 0, Math.PI * 2);
    ctx.fillStyle = color; ctx.shadowColor = color; ctx.shadowBlur = 6; ctx.fill();
    return new THREE.CanvasTexture(c);
  }

  function init() {
    var container = document.getElementById("cube-canvas-container");
    if (!container) { setTimeout(init, 300); return; }
    // If canvas exists AND is still in DOM, skip. Otherwise re-init.
    if (renderer && renderer.domElement && renderer.domElement.parentNode === container) return;
    // Clean up old renderer if page was navigated away and back
    if (renderer) { renderer.dispose(); renderer = null; }
    container.innerHTML = "";

    scene = new THREE.Scene();
    var fov = window.innerWidth < 768 ? 75 : 60;
    camera = new THREE.PerspectiveCamera(fov, window.innerWidth / window.innerHeight, 0.1, 2000);
    camera.position.set(0, 2, 12);

    renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setSize(window.innerWidth, window.innerHeight);
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    container.appendChild(renderer.domElement);

    // ── Hyper Core ──
    coreGroup = new THREE.Group();

    // Main cube with face textures
    var faces = [
      { emoji: "\u{1F4C8}", value: "$0.0245", label: "Floor Price", color: "#00dfc0" },
      { emoji: "\u{1F512}", value: "$72,600", label: "Treasury", color: "#6366f1" },
      { emoji: "\u{267E}\u{FE0F}", value: "AUTO", label: "Buyback", color: "#a78bfa" },
      { emoji: "\u{1F525}", value: "DEFLATION", label: "Burn", color: "#f472b6" },
      { emoji: "\u{2B21}", value: "", label: "Biyard", color: "#00dfc0" },
      { emoji: "\u{1F6E1}\u{FE0F}", value: "100%", label: "Protected", color: "#34d399" },
    ];
    var cubeMats = faces.map(function (f) {
      return new THREE.MeshPhongMaterial({
        map: makeFaceTexture(f.emoji, f.value, f.label, f.color),
        transparent: true, opacity: 0.8, shininess: 100,
        emissive: 0x00dfc0, emissiveIntensity: 0.05,
      });
    });
    var cubeGeo = new THREE.BoxGeometry(2.4, 2.4, 2.4);
    coreGroup.add(new THREE.Mesh(cubeGeo, cubeMats));

    // Inner nucleus
    coreGroup.add(new THREE.Mesh(
      new THREE.IcosahedronGeometry(0.7, 1),
      new THREE.MeshBasicMaterial({ color: 0x00dfc0, wireframe: true })
    ));

    // Outer wireframe
    coreGroup.add(new THREE.LineSegments(
      new THREE.EdgesGeometry(new THREE.BoxGeometry(2.5, 2.5, 2.5)),
      new THREE.LineBasicMaterial({ color: 0x00dfc0, transparent: true, opacity: 0.5 })
    ));

    // Hologram rings
    function makeRing(r, color, rx) {
      var m = new THREE.Mesh(
        new THREE.TorusGeometry(r, 0.015, 16, 100),
        new THREE.MeshBasicMaterial({ color: color, transparent: true, opacity: 0.3 })
      );
      m.rotation.x = rx;
      return m;
    }
    coreGroup.add(makeRing(3.2, 0x00dfc0, Math.PI / 2.5));
    coreGroup.add(makeRing(3.5, 0x7000ff, -Math.PI / 3));

    scene.add(coreGroup);

    // ── Warp Grid ──
    movingGrid = new THREE.Mesh(
      new THREE.PlaneGeometry(100, 100, 50, 50),
      new THREE.MeshBasicMaterial({ color: 0x00dfc0, wireframe: true, transparent: true, opacity: 0.07 })
    );
    movingGrid.rotation.x = -Math.PI / 2;
    movingGrid.position.y = -6;
    scene.add(movingGrid);

    // ── Orbiting brand entities ──
    var brands = [
      { name: "Shoe Brand", color: "#60a5fa", hex: 0x60a5fa },
      { name: "Coffee Brand", color: "#f472b6", hex: 0xf472b6 },
      { name: "Fashion Brand", color: "#34d399", hex: 0x34d399 },
      { name: "FreshMart", color: "#fbbf24", hex: 0xfbbf24 },
      { name: "FitLife", color: "#a78bfa", hex: 0xa78bfa },
      { name: "MediCare", color: "#38bdf8", hex: 0x38bdf8 },
      { name: "BrewHaus", color: "#00dfc0", hex: 0x00dfc0 },
      { name: "TechGym", color: "#7000ff", hex: 0x7000ff },
      { name: "RunClub", color: "#60a5fa", hex: 0x60a5fa },
      { name: "CafeNova", color: "#fbbf24", hex: 0xfbbf24 },
    ];

    brands.forEach(function (brand, i) {
      var group = new THREE.Group();
      var size = 0.2 + Math.random() * 0.1;
      var geo = i % 3 === 0
        ? new THREE.OctahedronGeometry(size)
        : new THREE.IcosahedronGeometry(size, 0);
      var tex = makeBrandTexture(brand.name, brand.color);
      var mat = new THREE.MeshPhongMaterial({
        map: i % 3 === 0 ? undefined : tex,
        color: i % 3 === 0 ? brand.hex : undefined,
        emissive: brand.hex, emissiveIntensity: 0.6, shininess: 100,
      });
      group.add(new THREE.Mesh(geo, mat));

      // Energy line
      var lineGeo = new THREE.BufferGeometry().setFromPoints([
        new THREE.Vector3(0, 0, 0), new THREE.Vector3(0, 0, 0),
      ]);
      var line = new THREE.Line(lineGeo, new THREE.LineBasicMaterial({
        color: brand.hex, transparent: true, opacity: 0.15,
      }));
      scene.add(line);

      orbitEntities.push({
        mesh: group, line: line,
        angle: (i / brands.length) * Math.PI * 2,
        radius: 5 + Math.random() * 3,
        speed: 0.002 + Math.random() * 0.005,
        yAmplitude: 2 + Math.random() * 2.5,
        yPhase: Math.random() * Math.PI,
      });
      scene.add(group);
    });

    // ── Particles ──
    var pGeo = new THREE.BufferGeometry();
    var pCount = 2500;
    var pPos = new Float32Array(pCount * 3);
    for (var i = 0; i < pCount * 3; i++) pPos[i] = (Math.random() - 0.5) * 50;
    pGeo.setAttribute("position", new THREE.BufferAttribute(pPos, 3));
    particles = new THREE.Points(pGeo, new THREE.PointsMaterial({
      size: 0.025, color: 0x00dfc0, transparent: true, opacity: 0.3,
      blending: THREE.AdditiveBlending,
    }));
    scene.add(particles);

    // ── Lights ──
    scene.add(new THREE.AmbientLight(0x060610, 2));
    var l1 = new THREE.PointLight(0x00dfc0, 2.5, 50);
    l1.position.set(10, 10, 10);
    scene.add(l1);
    var l2 = new THREE.PointLight(0x7000ff, 1.5, 30);
    l2.position.set(-8, -5, 5);
    scene.add(l2);

    window.addEventListener("resize", onResize);
    document.addEventListener("mousemove", onMove);
    document.addEventListener("touchmove", function (e) { onMove(e.touches[0]); }, false);
    window.addEventListener("scroll", onScroll);

    if (!animating) animate();
    animating = true;
  }

  function onMove(e) {
    targetX = (e.clientX - window.innerWidth / 2) / 100;
    targetY = (e.clientY - window.innerHeight / 2) / 100;
  }

  function onResize() {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.fov = window.innerWidth < 768 ? 75 : 60;
    camera.updateProjectionMatrix();
    renderer.setSize(window.innerWidth, window.innerHeight);
  }

  function onScroll() {
    var hero = document.querySelector('[data-animate="hero-cinematic"]');
    if (!hero) return;
    var rect = hero.getBoundingClientRect();
    var p = Math.max(0, Math.min(1, -rect.top / Math.max(1, rect.height - window.innerHeight)));
    scrollScale = 1 + p * 0.6;
    var about = document.getElementById("about");
    if (about && renderer) {
      var ar = about.getBoundingClientRect();
      var fade = Math.max(0, Math.min(1, (window.innerHeight - ar.top) / (window.innerHeight * 0.5)));
      renderer.domElement.style.opacity = String(1 - fade);
    }
  }

  function animate() {
    requestAnimationFrame(animate);
    if (!renderer || !scene || !camera || !coreGroup) return;
    var time = Date.now() * 0.001;

    // Camera smoothing
    mouseX += (targetX - mouseX) * 0.05;
    mouseY += (targetY - mouseY) * 0.05;
    camera.position.x += (mouseX - camera.position.x) * 0.05;
    camera.position.y += (-mouseY + 2 - camera.position.y) * 0.05;
    camera.lookAt(0, 0, 0);

    // Core rotation + pulse
    coreGroup.rotation.y += 0.006;
    coreGroup.rotation.z += 0.002;
    var pulse = 1 + Math.sin(time * 3) * 0.025;
    var s = scrollScale * pulse;
    coreGroup.scale.set(s, s, s);

    // Counter-rotating rings (children[3] and [4])
    if (coreGroup.children[3]) coreGroup.children[3].rotation.z += 0.008;
    if (coreGroup.children[4]) coreGroup.children[4].rotation.z -= 0.012;

    // Warp grid flow
    movingGrid.position.z += 0.12;
    if (movingGrid.position.z > 20) movingGrid.position.z = 0;

    // Orbiting entities + pulsing energy lines
    orbitEntities.forEach(function (ent) {
      ent.angle += ent.speed;
      var x = Math.cos(ent.angle) * ent.radius * scrollScale;
      var z = Math.sin(ent.angle) * ent.radius * scrollScale;
      var y = Math.sin(time * 0.5 + ent.yPhase) * ent.yAmplitude;
      ent.mesh.position.set(x, y, z);
      ent.mesh.rotation.y += 0.015;

      var pos = ent.line.geometry.attributes.position.array;
      pos[3] = x; pos[4] = y; pos[5] = z;
      ent.line.geometry.attributes.position.needsUpdate = true;
      ent.line.material.opacity = 0.08 + Math.sin(time * 5 + ent.angle) * 0.12;
    });

    particles.rotation.y += 0.0004;
    renderer.render(scene, camera);
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", function () { setTimeout(init, 200); });
  } else {
    setTimeout(init, 200);
  }
  var mo = new MutationObserver(function () {
    if (!document.querySelector("#cube-canvas-container canvas")) init();
  });
  mo.observe(document.body, { childList: true, subtree: true });
})();
