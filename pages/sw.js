// Cross-origin isolation, which SharedArrayBuffer requires and a
// static host will not give us.
//
// GitHub Pages serves what it is given and sets no headers of its
// own, so the page installs this worker to add them to its own
// responses and reloads once under it. That is the whole job; it
// caches nothing and changes nothing else.
self.addEventListener("install", () => self.skipWaiting());
self.addEventListener("activate", (event) => event.waitUntil(self.clients.claim()));

self.addEventListener("fetch", (event) => {
  const request = event.request;
  // A cache-only request from another origin cannot be re-fetched,
  // and answering it with a fetch would fail the navigation.
  if (request.cache === "only-if-cached" && request.mode !== "same-origin") return;
  event.respondWith(
    fetch(request).then((response) => {
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
    }),
  );
});
