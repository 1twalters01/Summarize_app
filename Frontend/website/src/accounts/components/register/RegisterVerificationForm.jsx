import { createSignal } from 'solid-js';
import { handlePostVerification } from '../../functions/register/handlers.js';

/** @typedef { import ('../../types/Props').RegisterProps } Props */

/** @param {Props} props */
const RegisterVerificationForm = (props) => {
  const [code, setCode] = createSignal('');

  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode?.()}>
        x
      </button>

      <form onSubmit={(e) => handlePostVerification(e, code(), props)}>
        <input
          type="text"
          placeholder="code"
          onInput={(e) => setCode(e.target.value)}
          required
        />
        <input type="submit" value="Submit" />
      </form>
    </>
  );
};

export default RegisterVerificationForm;
