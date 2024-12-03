import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { Layout } from './components/Layout';
import { LocoSplash } from './LocoSplash';
import { Dashboard } from './components/Dashboard';
import { NewPost } from './components/NewPost';
import { PostShow } from './components/PostShow';

const App = () => {
  return (
    <Router>
      <Layout>
        <Routes>
          <Route path="/" element={<LocoSplash />} />
          <Route path="/dashboard" element={<Dashboard />} />
          <Route path="/new-post" element={<NewPost />} />
          <Route path="/post/:id" element={<PostShow />} />
        </Routes>
      </Layout>
    </Router>
  );
};

export default App;
