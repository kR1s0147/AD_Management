// Listen for visited URLs using the History API.
chrome.history.onVisited.addListener(function(result) {
  console.log("Visited URL: " + result.url);
  chrome.storage.local.get(['visited'], function(data) {
    let visited = data.visited || [];
    visited.push({
      url: result.url,
      title: result.title || "",
      visitTime: result.lastVisitTime
    });
    chrome.storage.local.set({ visited: visited });
  });
});

// Listen for messages from the content script.
chrome.runtime.onMessage.addListener(function(message, sender, sendResponse) {
  if (message.type === "collectedContent") {
    chrome.storage.local.get(['collectedContent'], function(result) {
      let contentArr = result.collectedContent || [];
      contentArr.push(message.data);
      chrome.storage.local.set({ collectedContent: contentArr });
    });
  } else if (message.type === "searchQuery") {
    chrome.storage.local.get(['searchQueries'], function(result) {
      let queries = result.searchQueries || [];
      queries.push(message.data);
      chrome.storage.local.set({ searchQueries: queries });
    });
  }
});
