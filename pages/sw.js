// Cross-origin isolation, which SharedArrayBuffer requires and a
// static host will not give us, and the page with the network off.
//
// GitHub Pages serves what it is given and sets no headers of its
// own, so the page installs this worker to add them to its own
// responses and reloads once under it.
//
// It also keeps a copy of what it fetches, so that sw-apl runs with
// the network off: the interpreter is WebAssembly and local storage
// and reaches for nothing once it is here. The copy is only ever a
// fallback. Every request goes to the network first, as it always
// did, so a reader online always has the build that is published and
// never an older one a cache held on to -- the stale page that ignored
// typing was a cache answering first. Only when the network fails is
// the copy used.
//
// The copy is one cache per build, named for the build this worker
// was registered at (sw.js?v=...). A new build registers a new worker;
// when it takes over it deletes every older build's cache, and it has
// already filled its own, on installing, with the files the page needs.
// Entries are kept by address without the query, so the page's
// ?v=...&ts=... never stops an offline reload finding them.

const VERSION = new URL(self.location.href).searchParams.get("v") ?? "";
const CACHE = `sw-apl-${VERSION || "unstamped"}`;

// What the page loads, relative to where this worker is: enough for a
// session to start with the network off.
const SHELL = [
  "./", "version.txt", "build-info.json", "libraries.json", "manifest.json",
  "apl.js", "board.js", "worker.js", "wasm/apl_wasm.js", "wasm/apl_wasm_bg.wasm",
  "keymap.json", "glyph-names.json", "board-keys.json", "board.json",
  "redistributed/apl-keyboard/APL-keybd2.svg",
  "redistributed/apl-keyboard/APL-keybd2-board.svg",
  "redistributed/apl385-font/APL385.woff2",
  "favicon.ico", "icon-192.png",
];

// An address without its query: what the copy is kept under.
function key(url) {
  const bare = new URL(url, self.registration.scope);
  bare.search = "";
  return bare.href;
}

self.addEventListener("install", (event) => {
  event.waitUntil((async () => {
    const cache = await caches.open(CACHE);
    // Each file on its own, revalidated past the HTTP cache: one that
    // cannot be had now is fetched, and kept, when the page asks.
    await Promise.all(SHELL.map(async (path) => {
      try {
        const response = await fetch(new Request(key(path), { cache: "no-cache" }));
        if (response.ok) await cache.put(key(path), response);
      } catch { /* kept when it is next fetched */ }
    }));
    await self.skipWaiting();
  })());
});

self.addEventListener("activate", (event) => {
  event.waitUntil((async () => {
    for (const name of await caches.keys()) {
      if (name.startsWith("sw-apl-") && name !== CACHE) await caches.delete(name);
    }
    await self.clients.claim();
  })());
});

// The two headers, and the third a cross-origin resource needs.
function isolated(response) {
  if (response.status === 0) return response;
  const headers = new Headers(response.headers);
  headers.set("Cross-Origin-Opener-Policy", "same-origin");
  headers.set("Cross-Origin-Embedder-Policy", "require-corp");
  headers.set("Cross-Origin-Resource-Policy", "cross-origin");
  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers,
  });
}

// Whether a response is one to keep: a whole, readable answer to a GET.
function keepable(request, response) {
  return request.method === "GET" && response.status === 200
    && (response.type === "basic" || response.type === "cors");
}

self.addEventListener("fetch", (event) => {
  const request = event.request;
  // A cache-only request from another origin cannot be re-fetched,
  // and answering it with a fetch would fail the navigation.
  if (request.cache === "only-if-cached" && request.mode !== "same-origin") return;
  event.respondWith((async () => {
    try {
      const response = await fetch(request);
      if (keepable(request, response)) {
        const copy = response.clone();
        event.waitUntil(caches.open(CACHE).then((cache) => cache.put(key(request.url), copy)));
      }
      return isolated(response);
    } catch (error) {
      const kept = await (await caches.open(CACHE)).match(key(request.url));
      if (kept) return isolated(kept);
      throw error;
    }
  })());
});
