import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import ReactMarkdown from 'react-markdown';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { tomorrow } from 'react-syntax-highlighter/dist/esm/styles/prism';

interface Post {
  id: number;
  title: string;
  content: string;
  summary: string;
  published: boolean;
  user_id: string;
  created_at: string;
  published_at: string | null;
}

export const PostShow = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const [post, setPost] = useState<Post | null>(null);
  const [isAuthor, setIsAuthor] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchPost = async () => {
      try {
        const token = localStorage.getItem('token');
        if (!token) {
          navigate('/');
          return;
        }

        // Fetch post data
        const response = await fetch(`/api/posts/${id}`, {
          headers: {
            'Authorization': `Bearer ${token}`
          }
        });

        if (response.ok) {
          const postData = await response.json();
          setPost(postData);
          
          // Check if current user is the author
          const userResponse = await fetch('/api/auth/current', {
            headers: {
              'Authorization': `Bearer ${token}`
            }
          });
          
          if (userResponse.ok) {
            const userData = await userResponse.json();
            setIsAuthor(userData.pid === postData.user_id);
          }
        }
      } catch (error) {
        console.error('Error fetching post:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchPost();
  }, [id, navigate]);

  const handleEdit = () => {
    navigate(`/edit-post/${id}`);
  };

  if (loading) {
    return <div className="container mx-auto px-4 py-8">Loading...</div>;
  }

  if (!post) {
    return <div className="container mx-auto px-4 py-8">Post not found</div>;
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="max-w-3xl mx-auto">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-4xl font-bold">{post.title}</h1>
          {isAuthor && (
            <button
              onClick={handleEdit}
              className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md"
            >
              Edit Post
            </button>
          )}
        </div>

        <div className="mb-6 text-gray-600">
          <p>Published: {post.published_at ? new Date(post.published_at).toLocaleDateString() : 'Draft'}</p>
          <p>Created: {new Date(post.created_at).toLocaleDateString()}</p>
        </div>

        <div className="prose max-w-none">
          <div className="text-xl text-gray-600 mb-8">{post.summary}</div>
          <article className="markdown-content">
            <ReactMarkdown
              components={{
                code({node, inline, className, children, ...props}) {
                  const match = /language-(\w+)/.exec(className || '');
                  return !inline && match ? (
                    <SyntaxHighlighter
                      style={tomorrow}
                      language={match[1]}
                      PreTag="div"
                      {...props}
                    >
                      {String(children).replace(/\n$/, '')}
                    </SyntaxHighlighter>
                  ) : (
                    <code className={className} {...props}>
                      {children}
                    </code>
                  );
                }
              }}
            >
              {post.content}
            </ReactMarkdown>
          </article>
        </div>
      </div>
    </div>
  );
};
