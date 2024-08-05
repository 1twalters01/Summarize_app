const ChangeName = () => {
  const [firstName, setFirstName] = createSignal('');
  const [lastName, setLastName] = createSignal('');
  
  return (
    <>
      <h1>Change Name</h1>

      <form onSubmit={ (e) => handlePostName(e, password(), rememberMe(), setEmail) }>
        <input
          type="text"
          placeholder="first name"
          onInput={(e) => setFirstName(e.target.value)}
          required
        />
        <input
          type="checkbox"
          placeholder="last name"
          onInput={(e) => setLastName(e.target.checked)}
        />
        <input type="submit" value="Submit" />
      </form>
    </>
  );
};

export default ChangeName;
