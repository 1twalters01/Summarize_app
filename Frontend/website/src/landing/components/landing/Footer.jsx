const Footer = () => {
  return (
    <footer>
      <div class="icons">
        <a href="/">
          <img src="" />
        </a>

        <div id="out-links">
          <a href="">
            <i class="fa-brands fa-instagram"></i>
          </a>
          <a href="">
            <i class="fa-brands fa-facebook"></i>
          </a>
          <a href="">
            <i class="fa-brands fa-twitter"></i>
          </a>
          <a href="">
            <i class="fa-brands fa-youtube"></i>
          </a>
        </div>
      </div>

      <div>Languages</div>

      <div class="in-links">
        <div class="downloads">
          <h3>Download</h3>
          <a href="/downloads/mobile">iOS & Android</a>
          <a href="/downloads/desktop">Mac, Windows & Linux</a>
          <a href="/downloads/web-clipper">Web Clipper</a>
        </div>

        <div class="community">
          <h3>Community</h3>
          <a href="/blog">Blog</a>
          <a href="/community">Community</a>
          <a href="/webinars">Webinars</a>
        </div>

        <div class="products">
          <h3>Products</h3>
          <a href="/pricing">Pricing</a>
          <a href="/releases">Releases</a>
          <a href="/ai">AI</a>
          <a href="/library">Library</a>
          <a href="/sync">Sync</a>
          <a href="/code">Code</a>
        </div>

        <div>
          <h3>Company</h3>
          <a href="/about-us">About us</a>
          <a href="/email-us">Email us</a>
          <a href="/security">Security</a>
          <a href="/cookie-settings">Cookie settings</a>
          <a href="/terms">Terms & conditions</a>
          <a href="/privacy">Privacy</a>
        </div>
      </div>

      <div id="addendum">
        <div class="addendum-text">
          <p>We do not sell or share your personal information</p>
          <button>Cookie settings</button>
        </div>

        <p>&#169; Summarize, 2024</p>
      </div>
    </footer>
  );
};

export default Footer;
