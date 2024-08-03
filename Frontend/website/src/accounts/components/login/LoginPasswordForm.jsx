import { createSignal } from 'solid-js';
import { A, useNavigate } from '@solidjs/router';
import { useEmailContext } from '../../context/EmailContext';
import { handlePostPassword } from '../../functions/login/handlers';

/** @typedef { import('../../types/Props').LoginProps } Props */

/** @param {Props} props */
const LoginPasswordForm = (props) => {
  const [password, setPassword] = createSignal('');
  const [rememberMe, setRememberMe] = createSignal(false);
  const { setEmail } = useEmailContext();
  const navigate = useNavigate();

  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode?.()}>
        x
      </button>

      <form
        onSubmit={(e) =>
          handlePostPassword(
            e,
            password(),
            rememberMe(),
            setEmail,
            navigate,
            props
          )
        }
      >
        <input
          type="password"
          placeholder="password"
          onInput={(e) => setPassword(e.target.value)}
          required
        />
        <input
          type="checkbox"
          checked={rememberMe()}
          onChange={(e) => setRememberMe(e.target.checked)}
        />
        <input type="submit" value="Submit" />
      </form>

      <A href="/password-reset">Forgot your password?</A>
    </>
  );
};

export default LoginPasswordForm;
