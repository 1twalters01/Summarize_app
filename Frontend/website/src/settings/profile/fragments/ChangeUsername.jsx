import { createSignal } from 'solid-js';

const ChangeUsername = () => {
  const [username, setUsername] = createSignal('');
  
  return (
    <>
      <h1>Change Username</h1>

      <form onSubmit={(e) => handlePostUsername(e, username())}>
        <input
          type="text"
          placeholder="username"
          onInput={(e) => setUsername(e.target.value)}
          required
        />
        <input type="submit" value="Continue" />
      </form>
    </>
  );
};

export default ChangeUsername;
