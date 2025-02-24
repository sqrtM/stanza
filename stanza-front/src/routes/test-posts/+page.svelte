<script>
    import { onMount } from "svelte";

    let posts = [];
    let newPostContent = "";

    const apiUrl = "http://localhost:8080/api/v1/block";

    async function fetchPosts() {
        const response = await fetch(apiUrl);
        if (response.ok) {
            posts = await response.json();
        } else {
            console.error("Failed to fetch posts:", response.statusText);
        }
    }

    async function createPost() {
        const activityPubPost = {
            "@context": [
                {
                    target: "https://www.w3.org/ns/activitystreams",
                },
            ],
            type: "Create",
            object: {
                type: "Note",
                content: newPostContent,
                attributedTo: "meeee",
                published: Date.now().toLocaleString(),
            },
            actor: "meeee",
            to: ["public"],
        };

        const response = await fetch(apiUrl, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(activityPubPost),
        });

        if (response.ok) {
            newPostContent = "";
            fetchPosts();
        } else {
            console.error("Failed to create post:", response.statusText);
        }
    }

    onMount(() => {
        fetchPosts();
    });
</script>

<main>
    <h1>Posts</h1>

    <textarea bind:value={newPostContent} placeholder="Write your post here..."
    ></textarea>
    <button on:click={createPost}>Create Post</button>

    <h2>Existing Posts</h2>
    <ul>
        {#each posts as post}
            <li>{JSON.stringify(post.object)}</li>
        {/each}
    </ul>
</main>
