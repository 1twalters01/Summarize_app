const ChangeEmail = () => {
  const { email, setEmail } = useEmailContext();
  
  return (
    <>
      <h1>Change Email</h1>

      <form onSubmit={(e) => handlePostEmail(e, email(), props)}>
        <input
          type="email"
          placeholder="email"
          onInput={(e) => setEmail(e.target.value)}
          value={email()}
          required
        />
        <input type="submit" value="Continue" />
      </form>
    </>
  );
};

export default ChangeEmail;
