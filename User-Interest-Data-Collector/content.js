(function() {
    let startTime = Date.now();
    let maxScrollDepth = 0;
    let totalDwellTime = 0;
    let lastVisibilityTime = Date.now();
    let clickCount = 0;
  
    // Track scroll depth.
    window.addEventListener('scroll', function() {
      let scrollPosition = window.scrollY + window.innerHeight;
      maxScrollDepth = Math.max(maxScrollDepth, scrollPosition);
    });
  
    // Track clicks on the page.
    document.addEventListener('click', function() {
      clickCount++;
    });
  
    // Track dwell time using visibility change events.
    document.addEventListener('visibilitychange', function() {
      if (document.visibilityState === 'hidden') {
        totalDwellTime += Date.now() - lastVisibilityTime;
      } else {
        lastVisibilityTime = Date.now();
      }
    });
  
    // On page unload, finalize dwell time and send the collected data.
    window.addEventListener('beforeunload', function() {
      totalDwellTime += Date.now() - lastVisibilityTime;
      sendData();
    });
  
    // Also send data after a fixed interval (e.g., every 15 seconds).
    setTimeout(sendData, 15000);
  
    function sendData() {
      let title = document.title;
      let metaDescription = "";
      let metaTag = document.querySelector("meta[name='description']");
      if (metaTag && metaTag.content) {
        metaDescription = metaTag.content;
      }
      let snippet = document.body.innerText ? document.body.innerText.slice(0, 300) : "";
  
      // Capture device and environment metrics.
      let deviceMetrics = {
        width: window.innerWidth,
        height: window.innerHeight,
        language: navigator.language,
        userAgent: navigator.userAgent
      };
  
      let pageData = {
        url: window.location.href,
        title: title,
        metaDescription: metaDescription,
        snippet: snippet,
        maxScrollDepth: maxScrollDepth,
        dwellTime: totalDwellTime, // in milliseconds
        clickCount: clickCount,
        deviceMetrics: deviceMetrics,
        timestamp: Date.now()
      };
  
      // Send the page data to the background script.
      chrome.runtime.sendMessage({ type: "collectedContent", data: pageData });
  
      // Detect search queries on known search pages (Google and Bing).
      try {
        let urlObj = new URL(window.location.href);
        if ((urlObj.hostname.includes("google") && urlObj.pathname === "/search") ||
            (urlObj.hostname.includes("bing") && urlObj.pathname === "/search")) {
          let searchQuery = urlObj.searchParams.get("q");
          if (searchQuery) {
            chrome.runtime.sendMessage({
              type: "searchQuery",
              data: { query: searchQuery, url: window.location.href, timestamp: Date.now() }
            });
          }
        }
      } catch (e) {
        console.error("Error processing URL:", e);
      }
    }
  })();
  