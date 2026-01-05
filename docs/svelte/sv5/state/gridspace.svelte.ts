import { setContext, getContext } from 'svelte';
import type { Operator } from '$lib/types/gridspace/gridspace';

// Define the GridItem interface
export interface GridItem {
	id: string;
	x: number;
	y: number;
	w: number;
	h: number;
}

export class GridspaceState {
	// Grid items with reactive state
	items = $state<GridItem[]>([]);

	// Active/selected item
	activeItemId = $state<string | null>(null);

	// Grid configuration
	cols = $state<number>(12);
	itemSize = $state<{ height: number }>({ height: 40 });

	// Operator type
	operatorType = $state<Operator['type']>('Petrophysics');

	constructor(initialItems: GridItem[] = []) {
		if (initialItems.length > 0) {
			this.items = initialItems;
		}
	}

	// Add a new item to the grid
	addItem(id: string, x = 0, y = 0, w = 8, h = 20) {
		const newItem = { id, x, y, w, h };
		this.items = [...this.items, newItem];
	}

	// Remove an item from the grid
	removeItem(id: string) {
		this.items = this.items.filter((item) => item.id !== id);
	}

	// Update an item's position or size
	updateItem(id: string, updates: Partial<GridItem>) {
		this.items = this.items.map((item) => (item.id === id ? { ...item, ...updates } : item));
	}

	// Set the active item
	setActiveItem(id: string | null) {
		this.activeItemId = id;
	}

	// Get an item by id
	getItem(id: string) {
		return this.items.find((item) => item.id === id);
	}

	// Set operator type
	setOperatorType(type: Operator['type']) {
		this.operatorType = type;
	}

	// Update the grid layout when items are moved or resized
	updateLayout(newLayout: any) {
		// Ensure newLayout is an array
		if (!Array.isArray(newLayout)) {
			console.error('Expected newLayout to be an array, got:', newLayout);
			return;
		}

		// Update only position and size properties
		this.items = this.items.map((item) => {
			const updatedItem = newLayout.find((i) => i.id === item.id);
			if (updatedItem) {
				return {
					...item,
					x: updatedItem.x,
					y: updatedItem.y,
					w: updatedItem.w,
					h: updatedItem.h
				};
			}
			return item;
		});
	}
}

const GridspaceState_KEY = Symbol('GridspaceState');

export function setGridspaceState() {
	return setContext(GridspaceState_KEY, new GridspaceState());
}

export function getGridspaceState() {
	return getContext<ReturnType<typeof setGridspaceState>>(GridspaceState_KEY);
}
