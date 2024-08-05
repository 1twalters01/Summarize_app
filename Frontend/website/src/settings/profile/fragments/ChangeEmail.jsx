import { createSignal } from 'solid-js';

const ChangeEmail = () => {
  const [email, setEmail] = createSignal('');
  
  return (
    <>
      <h1>Change Email</h1>

      <form onSubmit={(e) => handlePostEmail(e, email())}>
        <input
          type="email"
          placeholder="email"
          onInput={(e) => setEmail(e.target.value)}
          required
        />
        <input type="submit" value="Continue" />
      </form>
    </>
  );
};

export default ChangeEmail;
