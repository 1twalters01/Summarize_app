import { Route } from '@solidjs/router';

import Main from './pages/Main';
import Contents from './pages/Contents';
import Chapter from './pages/Chapter';

let EditorRoutes = () => {
  return (
    <>
      <Route path="/editor" component={Main} />
      <Route path="/editor/contents" component={Contents} />
      <Route path="/editor/chapter" component={Chapter} />
    </>
  )
};

export default EditorRoutes;

