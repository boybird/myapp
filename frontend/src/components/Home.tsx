import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

interface Post {
  id: number;
  title: string;
  summary: string;
  published: boolean;
  published_at: string | null;
  created_at: string;
  updated_at: string;
}

interface PaginatedResponse {
  items: Post[];
  total: number;
  page: number;
  page_size: number;
  total_pages: number;
}

export const Home = () => {
  const [posts, setPosts] = useState<Post[]>([]);
  const [loading, setLoading] = useState(true);
  const [currentPage, setCurrentPage] = useState(1);
  const [pageSize] = useState(10);
  const [totalPages, setTotalPages] = useState(1);
  const navigate = useNavigate();

  useEffect(() => {
    fetchPosts();
  }, [currentPage, pageSize]);

  const fetchPosts = async () => {
    try {
      const response = await fetch(`/api/posts?page=${currentPage}&page_size=${pageSize}`);
      if (response.ok) {
        const data: PaginatedResponse = await response.json();
        setPosts(data.items);
        setTotalPages(data.total_pages);
      }
    } catch (error) {
      console.error('Error fetching posts:', error);
    } finally {
      setLoading(false);
    }
  };

  const handlePageChange = (newPage: number) => {
    setCurrentPage(newPage);
    window.scrollTo(0, 0);
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric'
    });
  };

  const renderPagination = () => {
    const pages = [];
    const maxVisiblePages = 5;
    let startPage = Math.max(1, currentPage - Math.floor(maxVisiblePages / 2));
    let endPage = Math.min(totalPages, startPage + maxVisiblePages - 1);

    if (endPage - startPage + 1 < maxVisiblePages) {
      startPage = Math.max(1, endPage - maxVisiblePages + 1);
    }

    for (let i = startPage; i <= endPage; i++) {
      pages.push(
        <button
          key={i}
          onClick={() => handlePageChange(i)}
          className={`px-3 py-1 mx-1 rounded ${
            currentPage === i
              ? 'bg-blue-500 text-white'
              : 'bg-gray-200 hover:bg-gray-300 text-black'
          }`}
        >
          {i}
        </button>
      );
    }

    return (
      <div className="flex justify-center items-center mt-8 space-x-2">
        <button
          onClick={() => handlePageChange(1)}
          disabled={currentPage === 1}
          className="px-3 py-1 rounded bg-gray-200 hover:bg-gray-300 disabled:opacity-50 text-black"
        >
          First
        </button>
        <button
          onClick={() => handlePageChange(currentPage - 1)}
          disabled={currentPage === 1}
          className="px-3 py-1 rounded bg-gray-200 hover:bg-gray-300 disabled:opacity-50 text-black"
        >
          Previous
        </button>
        {pages}
        <button
          onClick={() => handlePageChange(currentPage + 1)}
          disabled={currentPage === totalPages}
          className="px-3 py-1 rounded bg-gray-200 hover:bg-gray-300 disabled:opacity-50 text-black"
        >
          Next
        </button>
        <button
          onClick={() => handlePageChange(totalPages)}
          disabled={currentPage === totalPages}
          className="px-3 py-1 rounded bg-gray-200 hover:bg-gray-300 disabled:opacity-50 text-black"
        >
          Last
        </button>
      </div>
    );
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-4xl font-bold mb-8 text-center">Blog Posts</h1>
      {loading ? (
        <div className="text-center">Loading...</div>
      ) : (
        <>
          <div className="max-w-3xl mx-auto space-y-8">
            {posts.map((post) => (
              <article 
                key={post.id}
                className="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow cursor-pointer"
                onClick={() => navigate(`/post/${post.id}`)}
              >
                <h2 className="text-2xl font-bold mb-2 hover:text-blue-600 transition-colors">
                  {post.title}
                </h2>
                {post.published_at && (
                  <div className="text-gray-600 mb-3">
                    Published on {formatDate(post.published_at)}
                  </div>
                )}
                <p className="text-gray-700 mb-4">{post.summary}</p>
                <div className="text-blue-500 hover:text-blue-600">
                  Read more →
                </div>
              </article>
            ))}
          </div>
          {renderPagination()}
        </>
      )}
    </div>
  );
};
