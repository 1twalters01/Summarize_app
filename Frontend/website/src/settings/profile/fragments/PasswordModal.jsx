import { createSignal } from 'solid-js';

const PasswordModal = () => {
  const [password, setPassword] = createSignal('');

  return (
    <>
      <form onSubmit={(e) => handlePostPassword(e, password)}>
        <input
          type="password"
          placeholder="password"
          onInput={(e) => setPassword(e.target.value)}
          required
        />
      </form>
    </>
  );
};

export default PasswordModal;
