import { Route } from '@solidjs/router';

import Author from './pages/Author';
import User from './pages/User';

let AuthorsRoutes = () => {
  return (
    <>
      <Route path="author/:id" component={Author} />
      <Route path="user/:id" component={User} />
    </>
  );
};

export default AuthorsRoutes;
