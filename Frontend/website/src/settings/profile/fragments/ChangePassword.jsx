import { createSignal } from 'solid-js';

const ChangePassword = () => {
  const [password, setPassword] = createSignal('');
  const [passwordConfirmation, setPasswordConfirmation] = createSignal('');
  
  return (
    <>
      <h1>Change Password</h1>

      <form onSubmit={(e) => handlePostPasswords(e, password(), passwordConfirmation(), navigate)}>
        <input
          type="password"
          placeholder="password"
          onInput={(e) => setPassword(e.target.value)}
          required
        />
        <input
          type="password"
          placeholder="password confirmation"
          onInput={(e) => setPasswordConfirmation(e.target.value)}
          required
        />
        <input type="submit" value="Submit" />
      </form>
    </>
  );
};

export default ChangePassword;
