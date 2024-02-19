import { createSignal } from 'solid-js';
// import styles from './Accounts.module.css';


const Login = () => {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");

  /** @param {SubmitEvent} e */
  function PostLogin(e) {
      e.preventDefault();
      console.log("username: ", username());
      console.log("password: ", password());
      // json
  }

  return (
    // xsrf token?
    <>
      <h1>Login</h1>
      <form onSubmit={PostLogin} >
        <input
          type="text"
          placeholder="username"
          onInput={e => setUsername(e.target.value)} />
        <input
          type="password"
          placeholder="password"
          onInput={e => setPassword(e.target.value)} />
        <input type="submit" value="Login" />
      </form>
      <p>{username()}</p>
    </>
  );
};

export default Login;

