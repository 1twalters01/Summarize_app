import Navbar from "../components/navbar";

const Landing = () => {
  return (
    <>
      <Navbar />

      <div>
        <div>
          <h1>All of your knowledge, in one place</h1>
          <h2>Summarize helps you to improve your knowledge retention rate</h2>
        </div>

        <div>
          <a href="/login/">Log in</a>
          <a href="/register/">Sign up</a>
        </div>

        <footer>
          <div>
            <div>
              <a href="/">
                <img src="http://127.0.0.1:8080/" />
                <span>Summarize</span>
              </a>
            </div>

            <div>
              <a href=""><i class="fa-brands fa-instagram"></i></a>
              <a href=""><i class="fa-brands fa-facebook"></i></a>
              <a href=""><i class="fa-brands fa-twitter"></i></a>
              <a href=""><i class="fa-brands fa-youtube"></i></a>
            </div>

            <div>
              Languages
            </div>
          </div>


          <div>
            <div>
              <h3>Download</h3>
              <a href="/downloads/mobile">iOS & Android</a>
              <a href="/downloads/desktop">Mac, Windows & Linux</a>
              <a href="/downloads/web-clipper">Web Clipper</a>
            </div>

            <div>
              <h3>Community</h3>
              <a href="/blog">Blog</a>
              <a href="/community">Community</a>
              <a href="/webinars">Webinars</a>
            </div>
            <div>
              <h3><a href = "">Resources</a></h3>
              <a href="/pricing/">Pricing</a>
              <a href="">About us</a>
              <a href="">Email us</a>
              <a href="">Security</a>
              <a href="">Terms & conditions</a>
              <a href="">Privacy</a>
              <a href="">Cookie settings</a>
            </div>
          </div>

          <div>&#169; Summarize, Inc.</div>
        </footer>
      
      </div>
    </>
  )
};

export default Landing;
