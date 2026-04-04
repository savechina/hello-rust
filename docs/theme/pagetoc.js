// mdbook-pagetoc script
var anchors = [];
var position = 0;

function addAnchor(element) {
  let href = element.getAttribute('href');
  let text = element.textContent;
  let level = parseInt(element.tagName.substring(1));
  anchors.push({ href: href, text: text, level: level, element: element });
}

function init() {
  let sidebar = document.querySelector('.sidebar');
  let content = document.querySelector('.content');
  
  if (!sidebar || !content) return;
  
  // Create sidetoc container
  let sidetoc = document.createElement('div');
  sidetoc.className = 'sidetoc';
  
  let nav = document.createElement('nav');
  nav.className = 'pagetoc';
  sidetoc.appendChild(nav);
  
  sidebar.appendChild(sidetoc);
  
  // Collect all headers
  let headers = content.querySelectorAll('h1, h2, h3, h4');
  headers.forEach(function(header) {
    let anchor = header.querySelector('a.header');
    if (anchor) addAnchor(anchor);
  });
  
  // Build TOC
  anchors.forEach(function(a) {
    let link = document.createElement('a');
    link.href = a.href;
    link.className = 'pagetoc-H' + a.level;
    link.textContent = a.text;
    nav.appendChild(link);
  });
  
  // Scroll handler
  window.addEventListener('scroll', function() {
    let scroll = window.scrollY + 100;
    let active = null;
    
    anchors.forEach(function(a) {
      let elementTop = a.element.parentElement.offsetTop;
      if (scroll >= elementTop) {
        active = a.href;
      }
    });
    
    nav.querySelectorAll('a').forEach(function(link) {
      link.classList.remove('active');
      if (link.getAttribute('href') === active) {
        link.classList.add('active');
      }
    });
  });
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
