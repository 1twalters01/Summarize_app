import { A } from '@solidjs/router';

const Navbar = () => {
  return (
    <nav>
      <div>
        <A href="/">
          <img src="http://127.0.0.1:8080/favicon.ico" />
          <span>Summarize</span>
        </A>
      </div>

      <div>
        <A href="/pricing/">Pricing</A>
      </div>

      <div>
        <A href="/login/">Log in</A>
        <A href="/register/">Sign up</A>
      </div>
    </nav>
  );
};

export default Navbar;
