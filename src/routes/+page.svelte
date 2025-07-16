<script lang="ts">
  import { onMount } from 'svelte';
  import { torrentState } from '$lib/state/torrents.svelte';
  import { TorrentAPI } from '$lib/api/torrent.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card/index.js';
  import { Badge } from '$lib/components/ui/badge/index.js';
  import { Progress } from '$lib/components/ui/progress/index.js';
  import { Separator } from '$lib/components/ui/separator/index.js';
  import { Plus, Download, Upload, Users, Pause, Play, Trash2 } from '@lucide/svelte';

  let magnetUri = '';
  let isLoading = false;
  let error = '';

  // Load torrents on component mount
  onMount(async () => {
    await loadTorrents();
  });

  async function loadTorrents() {
    try {
      isLoading = true;
      const torrents = await TorrentAPI.getTorrents();
      torrentState.torrents = torrents;
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load torrents';
      console.error('Failed to load torrents:', err);
    } finally {
      isLoading = false;
    }
  }

  async function addTorrent() {
    if (!magnetUri.trim()) return;

    try {
      isLoading = true;
      error = '';
      
      await TorrentAPI.addTorrent(magnetUri);
      magnetUri = '';
      
      // Reload torrents to show the new one
      await loadTorrents();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to add torrent';
      console.error('Failed to add torrent:', err);
    } finally {
      isLoading = false;
    }
  }

  async function removeTorrent(torrentId: string) {
    try {
      await TorrentAPI.removeTorrent(torrentId);
      await loadTorrents();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to remove torrent';
      console.error('Failed to remove torrent:', err);
    }
  }

  async function pauseTorrent(torrentId: string) {
    try {
      await TorrentAPI.pauseTorrent(torrentId);
      await loadTorrents();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to pause torrent';
      console.error('Failed to pause torrent:', err);
    }
  }

  async function resumeTorrent(torrentId: string) {
    try {
      await TorrentAPI.resumeTorrent(torrentId);
      await loadTorrents();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to resume torrent';
      console.error('Failed to resume torrent:', err);
    }
  }

  function getStatusColor(status: string) {
    switch (status) {
      case 'downloading': return 'bg-blue-500';
      case 'seeding': return 'bg-green-500';
      case 'paused': return 'bg-yellow-500';
      case 'completed': return 'bg-emerald-500';
      case 'error': return 'bg-red-500';
      default: return 'bg-gray-500';
    }
  }

  function formatBytes(bytes: number) {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  function formatSpeed(bytesPerSecond: number) {
    return formatBytes(bytesPerSecond) + '/s';
  }
</script>

<svelte:head>
  <title>TorChat - Torrent Client with P2P Chat</title>
</svelte:head>

<div class="min-h-screen bg-background">
  <!-- Header -->
  <header class="border-b bg-card">
    <div class="container mx-auto px-4 py-6">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold tracking-tight">TorChat</h1>
          <p class="text-muted-foreground">Torrent Client with Integrated P2P Chat</p>
        </div>
        <div class="flex items-center gap-4">
          <Badge variant="outline" class="gap-2">
            <Download class="h-4 w-4" />
            {torrentState.torrents.filter(t => t.status === 'downloading').length} downloading
          </Badge>
          <Badge variant="outline" class="gap-2">
            <Upload class="h-4 w-4" />
            {torrentState.torrents.filter(t => t.status === 'seeding').length} seeding
          </Badge>
        </div>
      </div>
    </div>
  </header>

  <div class="container mx-auto px-4 py-6">
    <!-- Add Torrent Section -->
    <Card class="mb-6">
      <CardHeader>
        <CardTitle class="flex items-center gap-2">
          <Plus class="h-5 w-5" />
          Add Torrent
        </CardTitle>
        <CardDescription>
          Add a new torrent using a magnet link
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="flex gap-4">
          <Input
            bind:value={magnetUri}
            placeholder="magnet:?xt=urn:btih:..."
            class="flex-1"
            disabled={isLoading}
            onkeydown={(e) => e.key === 'Enter' && addTorrent()}
          />
          <Button 
            onclick={addTorrent} 
            disabled={isLoading || !magnetUri.trim()}
            class="min-w-24"
          >
            {isLoading ? 'Adding...' : 'Add'}
          </Button>
        </div>
        
        {#if error}
          <div class="mt-4 p-3 bg-destructive/10 border border-destructive/20 rounded-md">
            <p class="text-sm text-destructive">{error}</p>
          </div>
        {/if}
      </CardContent>
    </Card>

    <!-- Torrents List -->
    <div class="space-y-4">
      {#if torrentState.torrents.length === 0}
        <Card>
          <CardContent class="pt-6">
            <div class="text-center py-12">
              <div class="mx-auto h-12 w-12 text-muted-foreground mb-4">
                <Download class="h-full w-full" />
              </div>
              <h3 class="text-lg font-semibold mb-2">No torrents yet</h3>
              <p class="text-muted-foreground mb-4">Add your first torrent using a magnet link above</p>
            </div>
          </CardContent>
        </Card>
      {:else}
        {#each torrentState.torrents as torrent (torrent.id)}
          <Card>
            <CardContent class="pt-6">
              <div class="flex items-start justify-between mb-4">
                <div class="flex-1 min-w-0">
                  <h3 class="font-semibold truncate mb-1">{torrent.name}</h3>
                  <div class="flex items-center gap-2 text-sm text-muted-foreground">
                    <Badge 
                      variant="secondary" 
                      class="{getStatusColor(torrent.status)} text-white"
                    >
                      {torrent.status}
                    </Badge>
                    <span>{formatBytes(torrent.size)}</span>
                    {#if torrent.progress > 0}
                      <span>• {torrent.progress.toFixed(1)}%</span>
                    {/if}
                  </div>
                </div>
                
                <div class="flex items-center gap-2">
                  {#if torrent.status === 'paused'}
                    <Button 
                      variant="outline" 
                      size="sm"
                      onclick={() => resumeTorrent(torrent.id)}
                    >
                      <Play class="h-4 w-4" />
                    </Button>
                  {:else}
                    <Button 
                      variant="outline" 
                      size="sm"
                      onclick={() => pauseTorrent(torrent.id)}
                    >
                      <Pause class="h-4 w-4" />
                    </Button>
                  {/if}
                  
                  <Button 
                    variant="outline" 
                    size="sm"
                    onclick={() => removeTorrent(torrent.id)}
                  >
                    <Trash2 class="h-4 w-4" />
                  </Button>
                </div>
              </div>

              {#if torrent.progress > 0 && torrent.status !== 'completed'}
                <div class="mb-4">
                  <Progress value={torrent.progress} class="mb-2" />
                </div>
              {/if}

              <div class="grid grid-cols-1 md:grid-cols-4 gap-4 text-sm">
                <div class="flex items-center gap-2">
                  <Download class="h-4 w-4 text-blue-500" />
                  <span>{formatSpeed(torrent.downloadSpeed)}</span>
                </div>
                <div class="flex items-center gap-2">
                  <Upload class="h-4 w-4 text-green-500" />
                  <span>{formatSpeed(torrent.uploadSpeed)}</span>
                </div>
                <div class="flex items-center gap-2">
                  <Users class="h-4 w-4 text-purple-500" />
                  <span>{torrent.peers} peers</span>
                </div>
                <div class="text-muted-foreground">
                  Added: {new Date(torrent.addedAt).toLocaleDateString()}
                </div>
              </div>
            </CardContent>
          </Card>
        {/each}
      {/if}
    </div>
  </div>
</div>