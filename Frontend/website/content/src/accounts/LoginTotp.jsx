import { createSignal } from 'solid-js';
// import styles from './Accounts.module.css';


const LoginTotp = () => {
  const [totp, setTotp] = createSignal("");

  /** @param {SubmitEvent} e */
  function PostLogin(e) {
      e.preventDefault();
      console.log("totp: ", totp());
      // regular post
  }

  return (
    // xsrf token?
    <>
      <h1>Login Totp</h1>
      <form>
        <input
          type="number"
          placeholder="totp"
          onInput={e => setTotp(e.target.value)} />
        <input type="submit" value="Login" />
      </form>
      <p>{totp()}</p>
    </>
  );
};

export default LoginTotp;

