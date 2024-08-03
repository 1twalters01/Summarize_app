const Navbar = () => {
  return (
    <nav>
      <div>
        <a href="/">
          <img src="/favicon.ico" />
          <span>Summarize</span>
        </a>
      </div>

      <div>
        <a href="/pricing/">Pricing</a>
      </div>

      <div>
        <a href="/login/">Log in</a>
        <a href="/register/">Sign up</a>
      </div>
    </nav>
  );
};

export default Navbar;
