import { createSignal, Switch, Match, lazy } from 'solid-js';
import Navbar from '../navbar';
// const Navbar = lazy(() => import('../navbar'));
const LoginEmailForm = lazy(() => import('./LoginEmailForm'));
const LoginPasswordForm = lazy(() => import('./LoginPasswordForm'));
const LoginTotpForm = lazy(() => import('./LoginTotpForm'));

/** 
  * @template T
  * @typedef { import('solid-js').Accessor<T> } Accessor
*/

/**
  * @template T
  * @typedef { import('solid-js').Setter<T> } Setter
*/

/**
  * @template Y
  * @typedef { import('solid-js').Signal<Y> } Signal
*/

    
/** Enum for mode values.
  * @readonly
  * @enum {number}
  */
var modeOptions = {
  email: 0,
  password: 1,
  totp: 2,
};


const Login = () => {
  /** @type {Signal<modeOptions>} */
  const [mode, setMode] = createSignal(modeOptions.email);
  console.log(mode());

  // const emailMode = () => {
  //   setMode(modeOptions.email);
  // };

  const passwordMode = () => {
    setMode(modeOptions.password);
  };

  const totpMode = () => {
    setMode(modeOptions.totp);
  };

  return (
    <>
      <Navbar />

      <h1>Login</h1>
      
      <Switch>
        <Match when={mode() == modeOptions.email}>
          <LoginEmailForm passwordMode={passwordMode} />
        </Match>
        <Match when={mode() === modeOptions.password}>
          <LoginPasswordForm totpMode={totpMode} />
        </Match>
        <Match when={mode() === modeOptions.totp}>
          <LoginTotpForm />
        </Match>
      </Switch>
    </>
  );
};

export default Login;


