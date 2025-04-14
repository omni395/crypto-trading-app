<!-- src/components/chart/ObjectTree.vue -->
<template>
    <div
      v-if="showModal"
      @click.self="$emit('update:show-modal', false)"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    >
      <div class="bg-gray-800 p-4 rounded-lg w-96">
        <h2 class="text-white text-lg mb-4">Дерево объектов</h2>
        <div class="mb-4">
          <div v-for="obj in chartObjects" :key="obj.id" class="flex items-center gap-2 mb-2">
            <span class="text-white w-24">
              {{ obj.id === 'candlestick' ? 'График' : obj.id === 'volume' ? 'Объём' : obj.id }}
            </span>
            <button
              @click="$emit('toggle-visibility', obj)"
              :class="[
                'p-2 text-gray-400 hover:text-gray-200',
                obj.visible ? 'text-green-400' : 'text-red-400'
              ]"
              :title="obj.visible ? 'Скрыть' : 'Показать'"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 22"
                width="24"
                height="22"
                fill="none"
              >
                <g v-if="obj.visible" class="normal-eye">
                  <path
                    fill="currentColor"
                    fill-rule="evenodd"
                    clip-rule="evenodd"
                    d="M17.9948 7.91366C16.6965 6.48549 14.6975 5 11.9999 5C9.30225 5 7.30322 6.48549 6.00488 7.91366C6.00488 7.91366 4 10 4 11C4 12 6.00488 14.0863 6.00488 14.0863C7.30322 15.5145 9.30225 17 11.9999 17C14.6975 17 16.6965 15.5145 17.9948 14.0863C17.9948 14.0863 20 12 20 11C20 10 17.9948 7.91366 17.9948 7.91366ZM6.74482 13.4137C7.94648 14.7355 9.69746 16 11.9999 16C14.3022 16 16.0532 14.7355 17.2549 13.4137C17.2549 13.4137 19 11.5 19 11C19 10.5 17.2549 8.58634 17.2549 8.58634C16.0532 7.26451 14.3022 6 11.9999 6C9.69746 6 7.94648 7.26451 6.74482 8.58634C6.74482 8.58634 5 10.5 5 11C5 11.5 6.74482 13.4137 6.74482 13.4137Z"
                  />
                  <path
                    fill="currentColor"
                    fill-rule="evenodd"
                    clip-rule="evenodd"
                    d="M12 13C13.1046 13 14 12.1046 14 11C14 9.89543 13.1046 9 12 9C10.8954 9 10 9.89543 10 11C10 12.1046 10.8954 13 12 13ZM12 14C13.6569 14 15 12.6569 15 11C15 9.34315 13.6569 8 12 8C10.3431 8 9 9.34315 9 11C9 12.6569 10.3431 14 12 14Z"
                  />
                </g>
                <g v-else class="crossed-eye">
                  <path
                    fill="currentColor"
                    fill-rule="evenodd"
                    clip-rule="evenodd"
                    d="M8.8503 16.2712C9.76531 16.7135 10.8152 17 11.9999 17C14.6975 17 16.6965 15.5145 17.9948 14.0863C17.9948 14.0863 20 12 20 11C20 10 17.9948 7.91366 17.9948 7.91366C17.8729 7.77954 17.7448 7.64491 17.6105 7.51105L16.9035 8.2181C17.0254 8.33968 17.1425 8.46276 17.2549 8.58634C17.2549 8.58634 19 10.5 19 11C19 11.5 17.2549 13.4137 17.2549 13.4137C16.0532 14.7355 14.3022 16 11.9999 16C11.1218 16 10.324 15.8161 9.60627 15.5153L8.8503 16.2712ZM7.09663 13.7823C6.97455 13.6606 6.85728 13.5374 6.74482 13.4137C6.74482 13.4137 5 11.5 5 11C5 10.5 6.74482 8.58634 6.74482 8.58634C7.94648 7.26451 9.69746 6 11.9999 6C12.8781 6 13.6761 6.18398 14.394 6.48495L15.1499 5.729C14.2348 5.28657 13.1847 5 11.9999 5C9.30225 5 7.30322 6.48549 6.00488 7.91366C6.00488 7.91366 4 10 4 11C4 12 6.00488 14.0863 6.00488 14.0863C6.12693 14.2206 6.25516 14.3553 6.38959 14.4893L7.09663 13.7823Z"
                  />
                  <path
                    fill="currentColor"
                    fill-rule="evenodd"
                    clip-rule="evenodd"
                    d="M11.2231 13.8984C11.4709 13.9647 11.7313 14 12 14C13.6569 14 15 12.6569 15 11C15 9.34315 13.6569 8 12 8C12.2688 8 12.5294 8.03535 12.7772 8.10166L11.8751 9.00384C10.87 9.06578 10.0658 9.87001 10.0038 10.8751L9.10166 11.7772C9.03535 11.5294 9 11.2688 9 11C9 11C9 9.34315 10.3431 8 12 8C12.2688 8 12.5294 8.03535 12.7772 8.10166L11.8751 9.00384Z"
                  />
                  <path
                    fill="currentColor"
                    fill-rule="evenodd"
                    clip-rule="evenodd"
                    d="M5.64648 16.6465L17.6465 4.64648L18.3536 5.35359L6.35359 17.3536L5.64648 16.6465Z"
                  />
                </g>
              </svg>
            </button>
          </div>
        </div>
        <div v-for="line in drawnLines" :key="line.id" class="flex items-center gap-2 mb-2">
          <span class="text-white">
            {{ line.drawing_type === 'drawing.trendline' ? `Линия тренда` : `Уровень` }}:
            {{ line.drawing_type === 'drawing.trendline' ? `${line.data.start_price.toFixed(2)} - ${line.data.end_price.toFixed(2)}` : line.data.price.toFixed(2) }}
          </span>
          <button @click="$emit('remove-line', line.id)" class="p-1 text-white hover:text-gray-300">
            <svg
              width="16"
              height="16"
              viewBox="0 0 15 15"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              class="fill-current"
            >
              <path
                d="M12.8536 2.85355C13.0488 2.65829 13.0488 2.34171 12.8536 2.14645C12.6583 1.95118 12.3417 1.95118 12.1464 2.14645L7.5 6.79289L2.85355 2.14645C2.65829 1.95118 2.34171 1.95118 2.14645 2.14645C1.95118 2.34171 1.95118 2.65829 2.14645 2.85355L6.79289 7.5L2.14645 12.1536C1.95118 12.3488 1.95118 12.6654 2.14645 12.8606C2.34171 13.0558 2.65829 13.0558 2.85355 12.8606L7.5 8.20711L12.1464 12.8606C12.3417 13.0558 12.6583 13.0558 12.8536 12.8606C13.0488 12.6654 13.0488 12.3488 12.8536 12.1536L8.20711 7.5L12.8536 2.85355Z"
                fill="currentColor"
              />
            </svg>
          </button>
          <button @click="$emit('edit-line', line)" class="p-1 text-white hover:text-gray-300">
            <svg
              width="16"
              height="16"
              viewBox="0 0 15 15"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              class="fill-current"
            >
              <path
                d="M7.5 1C6.39543 1 5.5 1.89543 5.5 3C5.5 3.74028 5.93591 4.37338 6.56503 4.68282C5.60462 5.02251 4.89269 5.72727 4.572 6.65842C3.98187 6.31563 3.25 5.69382 3.25 5C3.25 3.89543 4.14543 3 5.25 3C5.59565 3 5.91766 3.10518 6.18616 3.289C6.07548 3.19115 5.94124 3.10991 5.79289 3.05051C5.32652 2.85885 5 2.46659 5 2C5 1.44772 5.44772 1 6 1H7.5ZM7.5 1C8.60457 1 9.5 1.89543 9.5 3C9.5 3.74028 9.06409 4.37338 8.43497 4.68282C9.39538 5.02251 10.1073 5.72727 10.428 6.65842C11.0181 6.31563 11.75 5.69382 11.75 5C11.75 3.89543 10.8546 3 9.75 3C9.40435 3 9.08234 3.10518 8.81384 3.289C8.92452 3.19115 9.05876 3.10991 9.20711 3.05051C9.67348 2.85885 10 2.46659 10 2C10 1.44772 9.55228 1 9 1H7.5ZM7.5 5C5.01472 5 3 7.01472 3 9.5C3 11.9853 5.01472 14 7.5 14C9.98528 14 12 11.9853 12 9.5C12 7.01472 9.98528 5 7.5 5ZM7.5 6C9.433 6 11 7.567 11 9.5C11 11.433 9.433 13 7.5 13C5.567 13 4 11.433 4 9.5C4 7.567 5.567 6 7.5 6Z"
                fill="currentColor"
              />
            </svg>
          </button>
        </div>
        <button @click="$emit('update:show-modal', false)" class="mt-4 p-2 bg-gray-700 text-white rounded hover:bg-gray-600">
          Закрыть
        </button>
      </div>
    </div>
  </template>
  
  <script>
  export default {
    name: 'ObjectTree',
    props: {
      showModal: {
        type: Boolean,
        required: true,
      },
      chartObjects: {
        type: Array,
        required: true,
      },
      drawnLines: {
        type: Array,
        required: true,
      },
    },
  };
  </script>