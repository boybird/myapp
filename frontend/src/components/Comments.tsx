import React, { useState, useEffect } from 'react';

interface Comment {
  id: number;
  content: string;
  user_id: string;
  post_id: number;
  parent_id: number | null;
  created_at: string;
  updated_at: string;
}

interface CommentsProps {
  postId: number;
}

export const Comments: React.FC<CommentsProps> = ({ postId }) => {
  const [comments, setComments] = useState<Comment[]>([]);
  const [newComment, setNewComment] = useState('');
  const [replyTo, setReplyTo] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchComments();
  }, [postId]);

  const fetchComments = async () => {
    try {
      const response = await fetch(`/api/comments?post_id=${postId}`);
      if (response.ok) {
        const data = await response.json();
        setComments(data);
      }
    } catch (error) {
      console.error('Error fetching comments:', error);
      setError('Failed to load comments');
    } finally {
      setLoading(false);
    }
  };

  const handleSubmitComment = async (e: React.FormEvent) => {
    e.preventDefault();
    
    const token = localStorage.getItem('token');
    if (!token) {
      setError('Please log in to comment');
      return;
    }

    try {
      const response = await fetch('/api/comments', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify({
          content: newComment,
          post_id: postId,
          parent_id: replyTo
        })
      });

      if (response.ok) {
        setNewComment('');
        setReplyTo(null);
        await fetchComments(); // Refresh comments
      } else {
        setError('Failed to post comment');
      }
    } catch (error) {
      console.error('Error posting comment:', error);
      setError('Failed to post comment');
    }
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  };

  const renderComment = (comment: Comment, level: number = 0) => {
    const replies = comments.filter(c => c.parent_id === comment.id);
    const maxIndentLevel = 3;
    const indentLevel = Math.min(level, maxIndentLevel);

    return (
      <div
        key={comment.id}
        className={`mb-4 ${indentLevel > 0 ? 'ml-8 border-l-2 border-gray-200 pl-4' : ''}`}
      >
        <div className="bg-white rounded-lg shadow-sm p-4">
          <div className="text-gray-700">{comment.content}</div>
          <div className="mt-2 text-sm text-gray-500">
            Posted on {formatDate(comment.created_at)}
          </div>
          <button
            onClick={() => setReplyTo(comment.id)}
            className="mt-2 text-blue-500 text-sm hover:text-blue-600"
          >
            Reply
          </button>
        </div>

        {replies.map(reply => renderComment(reply, level + 1))}
      </div>
    );
  };

  if (loading) {
    return <div className="mt-8">Loading comments...</div>;
  }

  return (
    <div className="mt-12">
      <h2 className="text-2xl font-bold mb-6">Comments</h2>
      
      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
          {error}
        </div>
      )}

      <form onSubmit={handleSubmitComment} className="mb-8">
        <div className="mb-4">
          {replyTo && (
            <div className="mb-2 text-sm text-gray-600">
              Replying to comment #{replyTo}
              <button
                onClick={() => setReplyTo(null)}
                className="ml-2 text-blue-500 hover:text-blue-600"
              >
                Cancel
              </button>
            </div>
          )}
          <textarea
            value={newComment}
            onChange={(e) => setNewComment(e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            rows={4}
            placeholder="Write a comment..."
            required
          />
        </div>
        <button
          type="submit"
          className="bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-600 transition-colors"
        >
          Post Comment
        </button>
      </form>

      <div className="space-y-6">
        {comments
          .filter(comment => !comment.parent_id) // Show only top-level comments
          .map(comment => renderComment(comment))}
      </div>
    </div>
  );
};
