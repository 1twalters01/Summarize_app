const Navbar = () => {
    return (
      <nav>
        <div>
          <a href="http://127.0.0.1:8080/">
            <img src="http://127.0.0.1:8080/favicon.ico" />
            <span>Summarize</span>
          </a>
        </div>

        <div>
          <a href="http://127.0.0.1:8080/pricing/">Pricing</a>
        </div>

        <div>
          <a href="http://127.0.0.1:8080/accounts/login/">Log in</a>
          <a href="http://127.0.0.1:8080/accounts/register/">Sign up</a>
       </div>
      </nav>
    )
}

export default Navbar;
