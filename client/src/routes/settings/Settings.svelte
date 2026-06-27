<script lang="ts">
  import Button from '$lib/components/primitives/Button.svelte';
  import Input from '$lib/components/primitives/Input.svelte';
  import Label from '$lib/components/primitives/Label.svelte';
  import Select from '$lib/components/primitives/Select.svelte';
  import Checkbox from '$lib/components/primitives/Checkbox.svelte';
  import Switch from '$lib/components/primitives/Switch.svelte';
  import Badge from '$lib/components/primitives/Badge.svelte';
  import Alert from '$lib/components/primitives/Alert.svelte';
  import Sidebar from '$lib/components/primitives/Sidebar.svelte';
  import SidebarMenuButton from '$lib/components/primitives/SidebarMenuButton.svelte';
  import SidebarMenuItem from '$lib/components/primitives/SidebarMenuItem.svelte';
  import SidebarMenuSection from '$lib/components/primitives/SidebarMenuSection.svelte';
  import SidebarMenuUser from '$lib/components/primitives/SidebarMenuUser.svelte';
  import Dropdown from '$lib/components/primitives/Dropdown.svelte';
  import DropdownItem from '$lib/components/primitives/DropdownItem.svelte';
  import DropdownLabel from '$lib/components/primitives/DropdownLabel.svelte';
  import DropdownSeparator from '$lib/components/primitives/DropdownSeparator.svelte';
  import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '$lib/components/primitives/card';
  import { Tabs, TabsList, TabsTrigger, TabsContent } from '$lib/components/primitives/tabs';
  import { getTheme } from '$lib/state/theme.svelte';

  const theme = getTheme();

  let sidebarOpen = $state(false);
  let checked1 = $state(false);
  let checked2 = $state(true);
  let switched1 = $state(false);
  let switched2 = $state(true);
  let selectValue = $state('');
  let activeNavItem = $state('models');
</script>

<div class="relative min-h-screen bg-background text-foreground">

  <!-- Sidebar demo -->
  <Sidebar bind:open={sidebarOpen} side="right" width="280px">
    {#snippet header()}
      <div class="p-2">
        <SidebarMenuButton name="OneVTT" subtitle="Game Master" onclick={() => {}}>
          {#snippet icon()}
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
              <path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/>
            </svg>
          {/snippet}
        </SidebarMenuButton>
      </div>
    {/snippet}

    <div class="flex-1 space-y-4 p-2">
      <SidebarMenuSection>
        <SidebarMenuItem arrow active={activeNavItem === 'models'} onclick={() => activeNavItem = 'models'}>
          {#snippet icon()}<svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/></svg>{/snippet}
          Scenes
        </SidebarMenuItem>
        <SidebarMenuItem arrow active={activeNavItem === 'actors'} onclick={() => activeNavItem = 'actors'}>
          {#snippet icon()}<svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>{/snippet}
          Actors
        </SidebarMenuItem>
        <SidebarMenuItem arrow active={activeNavItem === 'items'} onclick={() => activeNavItem = 'items'}>
          {#snippet icon()}<svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/></svg>{/snippet}
          Items
        </SidebarMenuItem>
      </SidebarMenuSection>

      <SidebarMenuSection label="Tools">
        <SidebarMenuItem active={activeNavItem === 'journal'} onclick={() => activeNavItem = 'journal'}>
          {#snippet icon()}<svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/></svg>{/snippet}
          Journal
        </SidebarMenuItem>
        <SidebarMenuItem active={activeNavItem === 'chat'} onclick={() => activeNavItem = 'chat'}>
          {#snippet icon()}<svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>{/snippet}
          Chat
        </SidebarMenuItem>
        <SidebarMenuItem active={activeNavItem === 'settings'} onclick={() => activeNavItem = 'settings'}>
          {#snippet icon()}<svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="3"/><path d="M19.07 4.93a10 10 0 0 1 0 14.14M4.93 4.93a10 10 0 0 0 0 14.14"/></svg>{/snippet}
          Settings
        </SidebarMenuItem>
      </SidebarMenuSection>
    </div>

    {#snippet footer()}
      <div class="p-2">
        <SidebarMenuUser name="Alex Davis" email="alex@example.com" onclick={() => {}} />
      </div>
    {/snippet}
  </Sidebar>

  <div class="mx-auto max-w-2xl px-8 py-10 space-y-6">

    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold">Settings</h1>
      <a href="/" class="text-sm text-muted-foreground hover:text-foreground transition-colors">← Back</a>
    </div>

    <Tabs value="theme">
      <TabsList>
        <TabsTrigger value="theme">Theme</TabsTrigger>
        <TabsTrigger value="controls">Controls</TabsTrigger>
        <TabsTrigger value="display">Display</TabsTrigger>
        <TabsTrigger value="layout">Layout</TabsTrigger>
      </TabsList>

      <!-- Theme -->
      <TabsContent value="theme" class="space-y-4">
        <h2 class="text-base font-semibold">Color theme</h2>
        <div class="flex gap-2">
          <Button variant={theme.current === 'light'   ? 'default' : 'outline'} onclick={() => theme.set('light')}>Light</Button>
          <Button variant={theme.current === 'dark'    ? 'default' : 'outline'} onclick={() => theme.set('dark')}>Dark</Button>
          <Button variant={theme.current === 'darkula' ? 'default' : 'outline'} onclick={() => theme.set('darkula')}>Darkula</Button>
        </div>
      </TabsContent>

      <!-- Controls -->
      <TabsContent value="controls" class="space-y-8">

        <section class="space-y-3">
          <h2 class="text-base font-semibold">Button</h2>
          <div class="flex flex-wrap gap-2">
            <Button>Default</Button>
            <Button variant="secondary">Secondary</Button>
            <Button variant="outline">Outline</Button>
            <Button variant="ghost">Ghost</Button>
            <Button variant="destructive">Destructive</Button>
          </div>
          <div class="flex flex-wrap gap-2 items-center">
            <Button size="sm">Small</Button>
            <Button size="default">Default</Button>
            <Button size="lg">Large</Button>
            <Button disabled>Disabled</Button>
          </div>
        </section>

        <div class="border-t border-border"></div>

        <section class="space-y-3">
          <h2 class="text-base font-semibold">Input &amp; Select</h2>
          <div class="space-y-1.5">
            <Label for="ex-name">Name</Label>
            <Input id="ex-name" placeholder="Your name…" />
          </div>
          <div class="space-y-1.5">
            <Label for="ex-select">Role</Label>
            <Select id="ex-select" bind:value={selectValue} class="max-w-xs">
              <option value="">Choose a role…</option>
              <option value="gm">Game Master</option>
              <option value="player">Player</option>
              <option value="observer">Observer</option>
            </Select>
          </div>
          <div class="space-y-1.5">
            <Label for="ex-disabled">Disabled</Label>
            <Input id="ex-disabled" placeholder="Cannot edit" disabled />
          </div>
        </section>

        <div class="border-t border-border"></div>

        <section class="space-y-3">
          <h2 class="text-base font-semibold">Checkbox &amp; Switch</h2>
          <div class="space-y-2">
            <label class="flex items-center gap-2 cursor-pointer text-sm">
              <Checkbox bind:checked={checked1} />
              <span>Show grid</span>
            </label>
            <label class="flex items-center gap-2 cursor-pointer text-sm">
              <Checkbox bind:checked={checked2} />
              <span>Enable fog of war</span>
            </label>
          </div>
          <div class="space-y-2 pt-1">
            <label class="flex items-center gap-3 cursor-pointer text-sm">
              <Switch bind:checked={switched1} />
              <span>Auto-save</span>
            </label>
            <label class="flex items-center gap-3 cursor-pointer text-sm">
              <Switch bind:checked={switched2} />
              <span>Sound effects</span>
            </label>
          </div>
        </section>

        <div class="border-t border-border"></div>

        <section class="space-y-3">
          <h2 class="text-base font-semibold">Dropdown</h2>
          <div class="flex gap-3">
            <Dropdown>
              {#snippet trigger()}
                <Button variant="outline">Options ↓</Button>
              {/snippet}
              <DropdownLabel>Actions</DropdownLabel>
              <DropdownSeparator />
              <DropdownItem>Edit</DropdownItem>
              <DropdownItem>Duplicate</DropdownItem>
              <DropdownSeparator />
              <DropdownItem variant="destructive">Delete</DropdownItem>
            </Dropdown>

            <Dropdown align="end">
              {#snippet trigger()}
                <Button variant="outline">Account ↓</Button>
              {/snippet}
              <DropdownLabel>alex@example.com</DropdownLabel>
              <DropdownSeparator />
              <DropdownItem>Profile</DropdownItem>
              <DropdownItem>Settings</DropdownItem>
              <DropdownSeparator />
              <DropdownItem variant="destructive">Log out</DropdownItem>
            </Dropdown>
          </div>
        </section>

      </TabsContent>

      <!-- Display -->
      <TabsContent value="display" class="space-y-8">

        <section class="space-y-3">
          <h2 class="text-base font-semibold">Card</h2>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <Card>
              <CardHeader>
                <CardTitle>Campaign Notes</CardTitle>
                <CardDescription>Session 12 — The Ruins of Ahm</CardDescription>
              </CardHeader>
              <CardContent>
                <p class="text-sm text-muted-foreground">The party discovered a hidden passage beneath the old keep…</p>
              </CardContent>
              <CardFooter class="gap-2">
                <Button size="sm">Edit</Button>
                <Button size="sm" variant="ghost">Share</Button>
              </CardFooter>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>World Stats</CardTitle>
                <CardDescription>Active session overview</CardDescription>
              </CardHeader>
              <CardContent class="space-y-2">
                <div class="flex items-center justify-between text-sm">
                  <span class="text-muted-foreground">Players connected</span>
                  <Badge>4</Badge>
                </div>
                <div class="flex items-center justify-between text-sm">
                  <span class="text-muted-foreground">Scene</span>
                  <span>The Crossroads</span>
                </div>
                <div class="flex items-center justify-between text-sm">
                  <span class="text-muted-foreground">Status</span>
                  <Badge variant="secondary">In progress</Badge>
                </div>
              </CardContent>
            </Card>
          </div>
        </section>

        <div class="border-t border-border"></div>

        <section class="space-y-3">
          <h2 class="text-base font-semibold">Badge</h2>
          <div class="flex flex-wrap gap-2">
            <Badge>Default</Badge>
            <Badge variant="secondary">Secondary</Badge>
            <Badge variant="outline">Outline</Badge>
            <Badge variant="destructive">Destructive</Badge>
          </div>
        </section>

        <div class="border-t border-border"></div>

        <section class="space-y-3">
          <h2 class="text-base font-semibold">Alert</h2>
          <Alert title="Heads up">
            This is an informational alert with a title and description.
          </Alert>
          <Alert variant="destructive" title="Something went wrong">
            Failed to connect to the server. Check that it's running on port 3000.
          </Alert>
        </section>

        <div class="border-t border-border"></div>

        <section class="space-y-3">
          <h2 class="text-base font-semibold">Tabs (nested)</h2>
          <Tabs value="a">
            <TabsList>
              <TabsTrigger value="a">Alpha</TabsTrigger>
              <TabsTrigger value="b">Beta</TabsTrigger>
              <TabsTrigger value="c" disabled>Disabled</TabsTrigger>
            </TabsList>
            <TabsContent value="a" class="text-sm text-muted-foreground">Content for Alpha tab.</TabsContent>
            <TabsContent value="b" class="text-sm text-muted-foreground">Content for Beta tab.</TabsContent>
          </Tabs>
        </section>

      </TabsContent>

      <!-- Layout -->
      <TabsContent value="layout" class="space-y-6">

        <section class="space-y-3">
          <h2 class="text-base font-semibold">Sidebar</h2>
          <p class="text-sm text-muted-foreground">
            Fixed overlay panel with workspace header, grouped nav items, and user footer. Open it to see the full component in context.
          </p>
          <Button variant="outline" onclick={() => sidebarOpen = !sidebarOpen}>
            {sidebarOpen ? 'Close sidebar' : 'Open sidebar'}
          </Button>
        </section>

        <div class="border-t border-border"></div>

        <section class="space-y-3">
          <h2 class="text-base font-semibold">Sidebar menu components</h2>
          <p class="text-sm text-muted-foreground">Previewed inline below.</p>
          <Card class="max-w-xs overflow-hidden">
            <div class="p-2 border-b border-border">
              <SidebarMenuButton name="OneVTT" subtitle="Game Master" onclick={() => {}}>
                {#snippet icon()}
                  <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                    <path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/>
                  </svg>
                {/snippet}
              </SidebarMenuButton>
            </div>
            <div class="p-2 space-y-3">
              <SidebarMenuSection>
                <SidebarMenuItem arrow active onclick={() => {}}>
                  {#snippet icon()}<svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/></svg>{/snippet}
                  Scenes
                </SidebarMenuItem>
                <SidebarMenuItem arrow onclick={() => {}}>
                  {#snippet icon()}<svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/></svg>{/snippet}
                  Actors
                </SidebarMenuItem>
              </SidebarMenuSection>
              <SidebarMenuSection label="Tools">
                <SidebarMenuItem onclick={() => {}}>
                  {#snippet icon()}<svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>{/snippet}
                  Chat
                </SidebarMenuItem>
                <SidebarMenuItem onclick={() => {}}>
                  {#snippet icon()}<svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><circle cx="12" cy="12" r="3"/><path d="M19.07 4.93a10 10 0 0 1 0 14.14M4.93 4.93a10 10 0 0 0 0 14.14"/></svg>{/snippet}
                  Settings
                </SidebarMenuItem>
              </SidebarMenuSection>
            </div>
            <div class="p-2 border-t border-border">
              <SidebarMenuUser name="Alex Davis" email="alex@example.com" onclick={() => {}} />
            </div>
          </Card>
        </section>

      </TabsContent>

    </Tabs>
  </div>
</div>
