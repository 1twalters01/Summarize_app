import { createSignal } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { useEmailContext } from '../../context/EmailContext';
import { handlePostTotp } from '../../functions/login/handlers';

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @typedef { import('../../types/Props').LoginProps } Props */

/** @param {Props} props */
const LoginTotpForm = (props) => {
  /** @type {[Accessor<number|null>, Setter<number|null>]} */
  const [digit1, setDigit1] = createSignal(null);
  /** @type {[Accessor<number|null>, Setter<number|null>]} */
  const [digit2, setDigit2] = createSignal(null);
  /** @type {[Accessor<number|null>, Setter<number|null>]} */
  const [digit3, setDigit3] = createSignal(null);
  /** @type {[Accessor<number|null>, Setter<number|null>]} */
  const [digit4, setDigit4] = createSignal(null);
  /** @type {[Accessor<number|null>, Setter<number|null>]} */
  const [digit5, setDigit5] = createSignal(null);
  /** @type {[Accessor<number|null>, Setter<number|null>]} */
  const [digit6, setDigit6] = createSignal(null);

  let { setEmail } = useEmailContext();
  const navigate = useNavigate();

  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode?.()}>
        x
      </button>

      <form
        onSubmit={(e) =>
          handlePostTotp(
            e,
            [
              digit1() ?? 11,
              digit2() ?? 11,
              digit3() ?? 11,
              digit4() ?? 11,
              digit5() ?? 11,
              digit6() ?? 11
            ],
            setEmail,
            navigate
          )
        }
      >
        <input
          type="text"
          onInput={(e) => setDigit1(+e.target.value)}
          required
        />
        <input
          type="text"
          onInput={(e) => setDigit2(+e.target.value)}
          required
        />
        <input
          type="text"
          onInput={(e) => setDigit3(+e.target.value)}
          required
        />
        <input
          type="text"
          onInput={(e) => setDigit4(+e.target.value)}
          required
        />
        <input
          type="text"
          onInput={(e) => setDigit5(+e.target.value)}
          required
        />
        <input
          type="text"
          onInput={(e) => setDigit6(+e.target.value)}
          required
        />
        <input type="submit" value="Submit" />
      </form>
    </>
  );
};

export default LoginTotpForm;
