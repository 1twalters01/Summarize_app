import { Route } from '@solidjs/router';

import Comments from './pages/Comments';
import Content from './pages/Content';
import Contents from './pages/Contents';
import Overview from './pages/Overview';


let AuthorsRoutes = () => {
  return (
    <>
      <Route path="author/:id" component={Comments} />
      <Route path="user/:id" component={Content} />
      <Route path="user/:id" component={Contents} />
      <Route path="user/:id" component={Overview} />
    </>
  )
};

export default AuthorsRoutes;

