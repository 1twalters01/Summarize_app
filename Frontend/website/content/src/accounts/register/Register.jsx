import { createSignal, Switch, Match, lazy } from 'solid-js';
import Navbar from '../navbar';
// const Navbar = lazy(() => import ('../navbar'));
const RegisterEmailForm = lazy(() => import('./RegisterEmailForm'));
const RegisterVerificationForm = lazy(() => import('./RegisterVerificationForm'));
const RegisterDetailsForm = lazy(() => import('./RegisterDetailsForm'));

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
  verify: 1,
  details: 2,
};


const Register = () => {
  /** @type {Signal<modeOptions>} */
  const [mode, setMode] = createSignal(modeOptions.email);
  console.log(mode());

  // const emailMode = () => {
  //   setMode(modeOptions.email);
  // };

  const verificationMode = () => {
    setMode(modeOptions.verify);
  };

  const detailsMode = () => {
    setMode(modeOptions.details);
  };

  return (
    <>
      <Navbar />

      <h1>Register</h1>

      <Switch>
        <Match when={modeOptions.email === mode()}>
          <RegisterEmailForm verificationMode={verificationMode} />
        </Match>
        <Match when={modeOptions.verify === mode()}>
          <RegisterVerificationForm detailsMode={detailsMode} />
        </Match>
        <Match when={modeOptions.details === mode()}>
          <RegisterDetailsForm />
        </Match>
      </Switch>
    </>
  );
};

export default Register;
