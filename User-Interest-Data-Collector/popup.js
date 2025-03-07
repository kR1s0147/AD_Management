document.getElementById('viewData').addEventListener('click', function() {
  chrome.storage.local.get(['visited', 'collectedContent', 'searchQueries'], function(data) {
    let resultDiv = document.getElementById('result');
    resultDiv.innerHTML = "<h3>Visited URLs:</h3>";
    resultDiv.innerHTML += `<pre>${JSON.stringify(data.visited || [], null, 2)}</pre>`;
    
    resultDiv.innerHTML += "<h3>Page Content Data:</h3>";
    resultDiv.innerHTML += `<pre>${JSON.stringify(data.collectedContent || [], null, 2)}</pre>`;
    
    resultDiv.innerHTML += "<h3>Search Queries:</h3>";
    resultDiv.innerHTML += `<pre>${JSON.stringify(data.searchQueries || [], null, 2)}</pre>`;
  });
});

document.getElementById('clearData').addEventListener('click', function() {
  if (confirm("Are you sure you want to clear all collected data?")) {
    chrome.storage.local.clear(function() {
      document.getElementById('result').innerHTML = "<p>Data cleared.</p>";
    });
  }
});

document.getElementById('downloadData').addEventListener('click', function() {
  chrome.storage.local.get(['visited', 'collectedContent', 'searchQueries'], function(data) {
    let combinedData = {
      visited: data.visited || [],
      collectedContent: data.collectedContent || [],
      searchQueries: data.searchQueries || []
    };
    let dataStr = JSON.stringify(combinedData, null, 2);
    let blob = new Blob([dataStr], { type: "application/json" });
    let url = URL.createObjectURL(blob);
    
    let a = document.createElement('a');
    a.href = url;
    a.download = "advanced_user_interest_data.json";
    a.click();
    URL.revokeObjectURL(url);
  });
});
