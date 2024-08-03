import { Route } from '@solidjs/router';

import Main from './pages/Main';
import Library from './pages/Library';

let AuthorsRoutes = () => {
  return (
    <>
      <Route path="/libraries" component={Main} />
      <Route path="/libraries/:id" component={Library} />
    </>
  );
};

export default AuthorsRoutes;
