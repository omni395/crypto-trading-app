<template>
  <div class="bg-custom">
    <div class="flex">
      <div class="w-10 bg-gray-800 h-[600px] flex flex-col items-center py-2">
        <button
          @click="toggleDrawing('line')"
          :class="[
            'p-2 text-gray-400 hover:text-gray-200',
            chartStore.drawingTool === 'line' ? 'text-green-400' : ''
          ]"
          title="Горизонтальный уровень"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28 28" width="28" height="28">
            <g fill="currentColor" fill-rule="nonzero">
              <path d="M4 15h8.5v-1h-8.5zM16.5 15h8.5v-1h-8.5z"></path>
              <path
                d="M14.5 16c.828 0 1.5-.672 1.5-1.5s-.672-1.5-1.5-1.5-1.5.672-1.5 1.5.672 1.5 1.5 1.5zm0 1c-1.381 0-2.5-1.119-2.5-2.5s1.119-2.5 2.5-2.5 2.5 1.119 2.5 2.5-1.119 2.5-2.5 2.5z"
              />
            </g>
          </svg>
        </button>
        <button
          @click="toggleDrawing('ray')"
          :class="[
            'p-2 text-gray-400 hover:text-gray-200',
            chartStore.drawingTool === 'ray' ? 'text-green-400' : ''
          ]"
          title="Горизонтальный луч"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28 28" width="28" height="28">
            <g fill="currentColor" fill-rule="nonzero">
              <path d="M8.5 15h16.5v-1h-16.5z"></path>
              <path
                d="M6.5 16c.828 0 1.5-.672 1.5-1.5s-.672-1.5-1.5-1.5-1.5.672-1.5 1.5.672 1.5 1.5 1.5zm0 1c-1.381 0-2.5-1.119-2.5-2.5s1.119-2.5 2.5-2.5 2.5 1.119 2.5 2.5-1.119 2.5-2.5 2.5z"
              />
            </g>
          </svg>
        </button>
        <button
          @click="toggleDrawing('trendline')"
          :class="[
            'p-2 text-gray-400 hover:text-gray-200',
            chartStore.drawingTool === 'trendline' ? 'text-green-400' : ''
          ]"
          title="Линия тренда"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28 28" width="28" height="28">
            <g fill="currentColor" fill-rule="nonzero">
              <path d="M7.354 21.354l14-14-.707-.707-14 14z"></path>
              <path
                d="M22.5 7c.828 0 1.5-.672 1.5-1.5s-.672-1.5-1.5-1.5-1.5.672-1.5 1.5.672 1.5 1.5 1.5zm0 1c-1.381 0-2.5-1.119-2.5-2.5s1.119-2.5 2.5-2.5 2.5 1.119 2.5 2.5-1.119 2.5-2.5 2.5zM5.5 24c.828 0 1.5-.672 1.5-1.5s-.672-1.5-1.5-1.5-1.5.672-1.5 1.5.672 1.5 1.5 1.5zm0 1c-1.381 0-2.5-1.119-2.5-2.5s1.119-2.5 2.5-2.5 2.5 1.119 2.5 2.5-1.119 2.5-2.5 2.5z"
              />
            </g>
          </svg>
        </button>
        <div class="flex-1"></div>
        <button
          @click="toggleMagnet"
          :class="[
            'p-2 text-gray-400 hover:text-gray-200',
            magnetEnabled ? 'text-green-400' : ''
          ]"
          title="Переключить магнит"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28 28" width="28" height="28">
            <g fill="currentColor" fill-rule="evenodd">
              <path
                fill-rule="nonzero"
                d="M14 10a2 2 0 0 0-2 2v11H6V12c0-4.416 3.584-8 8-8s8 3.584 8 8v11h-6V12a2 2 0 0 0-2-2zm-3 2a3 3 0 0 1 6 0v10h4V12c0-3.864-3.136-7-7-7s-7 3.136-7 7v10h4V12z"
              />
              <path d="M6.5 18h5v1h-5zm10 0h5v1h-5z" />
            </g>
          </svg>
        </button>
        <button
          @click="showModal = true"
          aria-label="Показать настройки графика"
          data-name="showObjectsTree"
          id="drawing-toolbar-object-tree"
          type="button"
          class="button-KTgbfaP5 apply-common-tooltip common-tooltip-vertical accessible-KTgbfaP5 text-gray-400 hover:text-gray-200"
          tabindex="-1"
          data-tooltip="Показать настройки графика"
        >
          <div class="bg-KTgbfaP5">
            <span class="icon-KTgbfaP5">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28 28" width="28" height="28" fill="none">
                <path
                  fill="currentColor"
                  fill-rule="evenodd"
                  clip-rule="evenodd"
                  d="M13 5.5c0-.28.22-.5.5-.5h1c.28 0 .5.22.5.5V7.05l.4.09c.9.18 1.73.53 2.46 1.02l.34.23.29-.3.81-.8c.2-.2.52-.2.71 0l.7.7.36-.35-.35.35c.2.2.2.51 0 .7l-.82.82-.29.29.23.34c.49.73.84 1.57 1.02 2.46l.08.4H22.5c.28 0 .5.22.5.5v1a.5.5 0 0 1-.5.5H20.95l-.09.4c-.18.9-.53 1.73-1.02 2.46l-.23.34.3.29.8.81c.2.2.2.52 0 .71l-.7.7a.5.5 0 0 1-.7 0l-.82-.8-.29-.3-.34.23c-.73.49-1.57.84-2.46 1.02l-.4.08V22.5a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5V20.95l-.4-.09a6.96 6.96 0 0 1-2.46-1.02l-.34-.23-.29.3-.81.8.35.36-.35-.35a.5.5 0 0 1-.71 0l-.7-.71a.5.5 0 0 1 0-.7l-.36-.36.35.35.82-.81.29-.29-.23-.34a6.96 6.96 0 0 1-1.02-2.46l-.08-.4H5.5a.5.5 0 0 1-.5-.5v-1c0-.28.22-.5.5-.5H7.05l.09-.4c.18-.9.53-1.73 1.02-2.46l.23-.34-.3-.29-.8-.81a.5.5 0 0 1 0-.71l.7-.7c.2-.2.51-.2.7 0l.82.8.29.3.34-.23a6.96 6.96 0 0 1 2.46-1.02l.4-.08V5.5zm.5-1.5c-.83 0-1.5.67-1.5 1.5v.75c-.73.2-1.43.48-2.06.86l-.54-.53a1.5 1.5 0 0 0-2.12 0l-.7.7a1.5 1.5 0 0 0 0 2.12l.53.54A7.95 7.95 0 0 0 6.25 12H5.5c-.83 0-1.5.67-1.5 1.5v1c0 .83.67 1.5 1.5 1.5h.75c.2.73.48 1.43.86 2.06l-.53.54a1.5 1.5 0 0 0 0 2.12l.7.7a1.5 1.5 0 0 0 2.12 0l.54-.53c.63.38 1.33.67 2.06.86v.75c0 .83.67 1.5 1.5 1.5h1c.83 0 1.5-.67 1.5-1.5v-.75a7.95 7.95 0 0 0 2.06-.86l.54.53a1.5 1.5 0 0 0 2.12 0l.7-.7a1.5 1.5 0 0 0 0-2.12l-.53-.54c.38-.63.67-1.33.86-2.06h.75c.83 0 1.5-.67 1.5-1.5v-1c0-.83-.67-1.5-1.5-1.5h-.75a7.95 7.95 0 0 0-.86-2.06l.53-.54a1.5 1.5 0 0 0 0-2.12l-.7-.7a1.5 1.5 0 0 0-2.12 0l-.54.53A7.95 7.95 0 0 0 16 6.25V5.5c0-.83-.67-1.5-1.5-1.5h-1zM12 14a2 2 0 1 1 4 0 2 2 0 0 1-4 0zm2-3a3 3 0 1 0 0 6 3 3 0 0 0 0-6z"
                />
              </svg>
            </span>
          </div>
        </button>
      </div>
      <div class="relative flex-1">
        <div id="chart-container" ref="chartContainer" class="w-full h-[600px]"></div>
        <button
          @click="scrollToLatestCandle"
          class="absolute top-2 right-2 p-2 bg-gray-700 text-white rounded hover:bg-gray-600 z-10"
          title="Вернуться к текущей свече"
        >
          <svg
            width="20"
            height="20"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            class="fill-current"
          >
            <path
              d="M6.70711 2.29289C6.31658 1.90237 5.68342 1.90237 5.29289 2.29289L0.792893 6.79289C0.402369 7.18342 0.402369 7.81658 0.792893 8.20711L5.29289 12.7071C5.68342 13.0976 6.31658 13.0976 6.70711 12.7071C7.09763 12.3166 7.09763 11.6834 6.70711 11.2929L4 8.5H13.5C14.0523 8.5 14.5 8.05228 14.5 7.5C14.5 6.94772 14.0523 6.5 13.5 6.5H4L6.70711 3.70711C7.09763 3.31658 7.09763 2.68342 6.70711 2.29289Z"
              fill="currentColor"
            />
          </svg>
        </button>
      </div>
    </div>

    <div
      v-if="showModal"
      @click.self="showModal = false"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    >
      <div class="bg-gray-800 p-4 rounded-lg w-96">
        <h2 class="text-white text-lg mb-4">Дерево объектов</h2>
        <div class="mb-4">
          <div v-for="obj in chartStore.chartObjects" :key="obj.id" class="flex items-center gap-2 mb-2">
            <span class="text-white w-24">
              {{ obj.id === 'candlestick' ? 'График' : obj.id === 'volume' ? 'Объём' : obj.id }}
            </span>
            <button
              @click="toggleVisibility(obj)"
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
                    d="M11.2231 13.8984C11.4709 13.9647 11.7313 14 12 14C13.6569 14 15 12.6569 15 11C15 10.7313 14.9647 10.4709 14.8984 10.2231L13.9961 11.1254C13.934 12.1301 13.1301 12.934 12.1254 12.9961L11.2231 13.8984ZM11.8751 9.00384C10.87 9.06578 10.0658 9.87001 10.0038 10.8751L9.10166 11.7772C9.03535 11.5294 9 11.2688 9 11C9 11C9 9.34315 10.3431 8 12 8C12.2688 8 12.5294 8.03535 12.7772 8.10166L11.8751 9.00384Z"
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
        <div v-for="line in chartStore.drawnLines" :key="line.id" class="flex items-center gap-2 mb-2">
          <span class="text-white">
            {{ line.drawing_type === 'drawing.trendline' ? `Линия тренда ${line.id.slice(0, 8)}` : `Уровень ${line.id.slice(0, 8)}` }}:
            {{ line.drawing_type === 'drawing.trendline' ? `${line.data.start_price.toFixed(2)} - ${line.data.end_price.toFixed(2)}` : line.data.price.toFixed(2) }}
          </span>
          <button @click="removeDrawnLine(line)" class="p-1 text-white hover:text-gray-300">
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
          <button @click="openLineProperties(line)" class="p-1 text-white hover:text-gray-300">
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
        <button @click="showModal = false" class="mt-4 p-2 bg-gray-700 text-white rounded hover:bg-gray-600">
          Закрыть
        </button>
      </div>
    </div>

    <div
      v-if="showPropertiesModal"
      @click.self="showPropertiesModal = false"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    >
      <div class="bg-gray-800 p-0 rounded-lg w-96">
        <div class="flex justify-between items-center p-2 border-b border-gray-600">
          <h2 class="text-white text-lg">Свойства линии</h2>
          <button @click="showPropertiesModal = false" class="text-gray-400 hover:text-gray-200">
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
        </div>
        <div class="p-2">
          <div class="flex items-center gap-2 mb-2">
            <label class="text-white w-24">Цвет:</label>
            <input
              type="color"
              v-model="selectedLine.data.color"
              @change="updateLineProperties"
              class="w-10 h-6 rounded"
            />
          </div>
          <div class="flex items-center gap-2 mb-2">
            <label class="text-white w-24">Толщина:</label>
            <select
              v-model.number="selectedLine.data.line_width"
              @change="updateLineProperties"
              class="p-1 bg-gray-700 text-white rounded w-16"
            >
              <option v-for="size in [1, 2, 3, 4, 5]" :key="size" :value="size">{{ size }}px</option>
            </select>
          </div>
          <div v-if="selectedLine.drawing_type !== 'drawing.trendline'" class="flex items-center gap-2 mb-2">
            <label class="text-white w-24">Цена:</label>
            <input
              type="number"
              v-model.number="selectedLine.data.price"
              @change="updateLineProperties"
              step="0.01"
              class="p-1 bg-gray-700 text-white rounded w-32"
            />
          </div>
          <div v-if="selectedLine.drawing_type === 'drawing.trendline'" class="flex items-center gap-2 mb-2">
            <label class="text-white w-24">Начальная цена:</label>
            <input
              type="number"
              v-model.number="selectedLine.data.start_price"
              @change="updateLineProperties"
              step="0.01"
              class="p-1 bg-gray-700 text-white rounded w-32"
            />
          </div>
          <div v-if="selectedLine.drawing_type === 'drawing.trendline'" class="flex items-center gap-2 mb-2">
            <label class="text-white w-24">Конечная цена:</label>
            <input
              type="number"
              v-model.number="selectedLine.data.end_price"
              @change="updateLineProperties"
              step="0.01"
              class="p-1 bg-gray-700 text-white rounded w-32"
            />
          </div>
          <button
            @click="removeDrawnLine(selectedLine); showPropertiesModal = false"
            class="p-2 bg-red-600 text-white rounded hover:bg-red-700"
            title="Удалить линию"
          >
            <svg
              width="16"
              height="16"
              viewBox="0 0 15 15"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              class="fill-current"
            >
              <path
                d="M5 3V2C5 1.44772 5.44772 1 6 1H9C9.55228 1 10 1.44772 10 2V3H12.5C12.7761 3 13 3.22386 13 3.5C13 3.77614 12.7761 4 12.5 4H12V12C12 12.5523 11.5523 13 11 13H4C3.44772 13 3 12.5523 3 12V4H2.5C2.22386 4 2 3.77614 2 3.5C2 3.22386 2.22386 3 2.5 3H5ZM6 2V3H9V2H6ZM4 4V12H11V4H4ZM6 6C6 5.44772 6.44772 5 7 5C7.55228 5 8 5.44772 8 6V10C8 10.5523 7.55228 11 7 11C6.44772 11 6 10.5523 6 10V6Z"
                fill="currentColor"
              />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import * as LightweightCharts from 'lightweight-charts';
const { createChart, CrosshairMode, LineStyle } = LightweightCharts;

import { useChartStore } from '@/stores/chart';

export default {
  name: "Chart",
  setup() {
    const chartStore = useChartStore();
    return { chartStore };
  },
  data() {
    return {
      showModal: false,
      showPropertiesModal: false,
      selectedLine: null,
      trendlineStart: null,
      magnetEnabled: false,
    };
  },
  async mounted() {
    try {
      this.initChart();
      await this.setupWebSocket();
      this.requestHistoricalData();
      this.loadDrawingLines();
    } catch (error) {
      console.error("Error in mounted hook:", error);
    }
  },
  methods: {
    initChart() {
      const chartContainer = this.$refs.chartContainer;
      if (!chartContainer) {
        console.error("Chart container not found");
        return;
      }

      const width = chartContainer.clientWidth || window.innerWidth;
      const height = chartContainer.clientHeight || 600;
      if (width === 0 || height === 0) {
        console.error("Chart container has invalid dimensions:", { width, height });
        return;
      }

      const chart = createChart(chartContainer, {
        width: width,
        height: height,
        layout: {
          background: { color: "rgb(32, 41, 56)" },
          textColor: "#DDD",
        },
        grid: {
          vertLines: { color: "#444" },
          horzLines: { color: "#444" },
        },
        crosshair: {
          mode: CrosshairMode.Normal,
          vertLine: {
            labelVisible: false,
          },
          horzLine: {
            labelVisible: false,
          },
        },
        timeScale: {
          timeVisible: true,
          secondsVisible: false,
        },
        rightPriceScale: {
          borderVisible: false,
        },
      });

      this.chartStore.setChart(chart);

      const candlestickSeries = chart.addCandlestickSeries({
        upColor: "#26a69a",
        downColor: "#ef5350",
        wickUpColor: "#26a69a",
        wickDownColor: "#ef5350",
      });

      candlestickSeries.priceScale().applyOptions({
        scaleMargins: {
          top: 0.1,
          bottom: 0.4,
        },
      });

      this.chartStore.addChartObject({
        id: "candlestick",
        type: "candlestick",
        series: candlestickSeries,
        visible: true,
        settings: { upColor: "#26a69a", downColor: "#ef5350" },
      });

      const volumeSeries = chart.addHistogramSeries({
        color: "#26a69a",
        priceFormat: {
          type: "volume",
        },
        priceScaleId: "",
        lastValueVisible: true,
      });

      volumeSeries.priceScale().applyOptions({
        scaleMargins: {
          top: 0.7,
          bottom: 0,
        },
      });

      this.chartStore.addChartObject({
        id: "volume",
        type: "volume",
        series: volumeSeries,
        visible: true,
        settings: {},
      });

      chart.timeScale().subscribeVisibleTimeRangeChange(() => {
        const timeRange = chart.timeScale().getVisibleLogicalRange();
        if (timeRange && timeRange.from < 0 && !this.chartStore.isLoading) {
          this.loadMoreHistoricalData();
        }
      });

      chartContainer.addEventListener("contextmenu", (e) => {
        e.preventDefault();
        if (this.chartStore.drawingTool) {
          this.chartStore.setDrawingTool(null);
          this.trendlineStart = null;
        }
      });

      chart.subscribeClick((param) => {
        if (!param.point) return;

        const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
        if (!candlestickObj || !candlestickObj.visible) return;

        let price = candlestickObj.series.coordinateToPrice(param.point.y);
        const time = chart.timeScale().coordinateToTime(param.point.x);

        if (this.magnetEnabled) {
          price = this.getMagnetPrice(candlestickObj.series, price, time);
        }

        if (this.chartStore.drawingTool === "line") {
          const line = candlestickObj.series.createPriceLine({
            price: price,
            color: "#FFD700",
            lineWidth: 1,
            lineStyle: LineStyle.Dashed,
          });
          const drawingData = {
            price: price,
            color: "#FFD700",
            line_width: 1,
          };
          this.chartStore.addDrawnLine({
            id: "",
            drawing_type: "drawing.line",
            data: drawingData,
            line: line,
          });
          this.saveDrawingLine("drawing.line", drawingData);
          this.chartStore.setDrawingTool(null);
        } else if (this.chartStore.drawingTool === "ray") {
          const raySeries = chart.addLineSeries({
            color: "#FFD700",
            lineWidth: 1,
            lineStyle: LineStyle.Dashed,
            priceLineVisible: false,
            lastValueVisible: false,
          });

          const lastTime = this.getLastVisibleTime();
          raySeries.setData([
            { time: time, value: price },
            { time: lastTime, value: price },
          ]);

          const drawingData = {
            price: price,
            start_time: time * 1000,
            color: "#FFD700",
            line_width: 1,
          };
          this.chartStore.addDrawnLine({
            id: "",
            drawing_type: "drawing.ray",
            data: drawingData,
            line: raySeries,
          });
          this.saveDrawingLine("drawing.ray", drawingData);
          this.chartStore.setDrawingTool(null);
        } else if (this.chartStore.drawingTool === "trendline") {
          if (!this.trendlineStart) {
            this.trendlineStart = { price, time };
          } else {
            let startPrice = this.trendlineStart.price;
            let endPrice = price;
            if (this.magnetEnabled) {
              startPrice = this.getMagnetPrice(candlestickObj.series, startPrice, this.trendlineStart.time);
              endPrice = this.getMagnetPrice(candlestickObj.series, endPrice, time);
            }

            const trendlineSeries = chart.addLineSeries({
              color: "#FFD700",
              lineWidth: 1,
              lineStyle: LineStyle.Solid,
              priceLineVisible: false,
              lastValueVisible: false,
            });

            trendlineSeries.setData([
              { time: this.trendlineStart.time, value: startPrice },
              { time: time, value: endPrice },
            ]);

            const drawingData = {
              start_price: startPrice,
              end_price: endPrice,
              start_time: this.trendlineStart.time * 1000,
              end_time: time * 1000,
              color: "#FFD700",
              line_width: 1,
            };

            this.chartStore.addDrawnLine({
              id: "",
              drawing_type: "drawing.trendline",
              data: drawingData,
              line: trendlineSeries,
            });

            this.saveDrawingLine("drawing.trendline", drawingData);

            this.trendlineStart = null;
            this.chartStore.setDrawingTool(null);
          }
        } else {
          const line = this.chartStore.drawnLines.find((line) =>
            this.isPointNearLine(line, price, time)
          );
          if (line) {
            this.selectedLine = line;
            this.openLineProperties(line);
          } else {
            this.selectedLine = null;
          }
        }
      });
    },
    getMagnetPrice(series, price, clickTime) {
      const data = series.data();
      if (data.length === 0) return price;

      const nearestCandle = data.reduce((closest, candle) => {
        const timeDiff = Math.abs(candle.time - clickTime);
        const closestTimeDiff = Math.abs(closest.time - clickTime);
        return timeDiff < closestTimeDiff ? candle : closest;
      }, data[0]);

      const prices = [
        nearestCandle.open,
        nearestCandle.high,
        nearestCandle.low,
        nearestCandle.close,
      ];
      return prices.reduce((closestPrice, p) => {
        return Math.abs(p - price) < Math.abs(closestPrice - price) ? p : closestPrice;
      }, prices[0]);
    },
    getLastVisibleTime() {
      const visibleRange = this.chartStore.chart.timeScale().getVisibleRange();
      if (!visibleRange) {
        return Math.floor(Date.now() / 1000);
      }
      return visibleRange.to;
    },
    isPointNearLine(line, price, time) {
      if (line.drawing_type === "drawing.line") {
        return Math.abs(line.data.price - price) < 1.5;
      } else if (line.drawing_type === "drawing.ray") {
        if (time < line.data.start_time / 1000) return false;
        return Math.abs(line.data.price - price) < 1.5;
      } else if (line.drawing_type === "drawing.trendline") {
        const t = (time - line.data.start_time / 1000) / (line.data.end_time / 1000 - line.data.start_time / 1000);
        if (t < 0 || t > 1) return false;
        const interpolatedPrice = line.data.start_price + t * (line.data.end_price - line.data.start_price);
        return Math.abs(price - interpolatedPrice) < 1.5;
      }
      return false;
    },
    toggleDrawing(tool) {
      const newTool = this.chartStore.drawingTool === tool ? null : tool;
      this.chartStore.setDrawingTool(newTool);
      if (newTool !== "trendline") {
        this.trendlineStart = null;
      }
    },
    toggleMagnet() {
      this.magnetEnabled = !this.magnetEnabled;
    },
    toggleVisibility(obj) {
      this.chartStore.toggleObjectVisibility(obj.id);
      const updatedObj = this.chartStore.chartObjects.find((o) => o.id === obj.id);
      updatedObj.series.applyOptions({ visible: updatedObj.visible });
      if (updatedObj.id === "volume" && updatedObj.visible) {
        this.updateVolumeLabel();
      }
    },
    updateVolumeLabel() {
      const volumeObj = this.chartStore.chartObjects.find((obj) => obj.id === "volume");
      if (volumeObj && volumeObj.visible) {
        const volumeData = volumeObj.series.data();
        if (volumeData.length > 0) {
          const lastVolume = volumeData[volumeData.length - 1];
          const color = lastVolume.color || (this.chartStore.lastCandle.close >= this.chartStore.lastCandle.open ? "#26a69a" : "#ef5350");
          volumeObj.series.applyOptions({
            lastValueVisible: true,
            priceLineVisible: false,
            lastPriceAnimation: 0,
            color: color,
          });
        }
      }
    },
    removeDrawnLine(line) {
      if (line.line) {
        this.chartStore.chart.removeSeries(line.line);
        this.deleteDrawingLine(line.drawing_type, line.id);
      }
      this.chartStore.removeDrawnLine(this.chartStore.drawnLines.findIndex(l => l.id === line.id));
      if (this.selectedLine && this.selectedLine.id === line.id) {
        this.selectedLine = null;
      }
    },
    openLineProperties(line) {
      this.selectedLine = { ...line, data: { ...line.data } };
      this.showPropertiesModal = true;
    },
    updateLineProperties() {
      const line = this.chartStore.drawnLines.find(l => l.id === this.selectedLine.id);
      if (!line) return;

      if (line.line) {
        this.chartStore.chart.removeSeries(line.line);
      }

      const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
      let updatedLine;
      if (line.drawing_type === "drawing.line") {
        updatedLine = candlestickObj.series.createPriceLine({
          price: this.selectedLine.data.price,
          color: this.selectedLine.data.color,
          lineWidth: this.selectedLine.data.line_width,
          lineStyle: LineStyle.Dashed,
        });
      } else if (line.drawing_type === "drawing.ray") {
        updatedLine = this.chartStore.chart.addLineSeries({
          color: this.selectedLine.data.color,
          lineWidth: this.selectedLine.data.line_width,
          lineStyle: LineStyle.Dashed,
          priceLineVisible: false,
          lastValueVisible: false,
        });
        updatedLine.setData([
          { time: line.data.start_time / 1000, value: this.selectedLine.data.price },
          { time: this.getLastVisibleTime(), value: this.selectedLine.data.price },
        ]);
      } else if (line.drawing_type === "drawing.trendline") {
        updatedLine = this.chartStore.chart.addLineSeries({
          color: this.selectedLine.data.color,
          lineWidth: this.selectedLine.data.line_width,
          lineStyle: LineStyle.Solid,
          priceLineVisible: false,
          lastValueVisible: false,
        });
        updatedLine.setData([
          { time: line.data.start_time / 1000, value: this.selectedLine.data.start_price },
          { time: line.data.end_time / 1000, value: this.selectedLine.data.end_price },
        ]);
      }

      this.chartStore.updateDrawnLine(this.chartStore.drawnLines.findIndex(l => l.id === line.id), {
        ...line,
        data: { ...this.selectedLine.data },
        line: updatedLine,
      });

      this.saveDrawingLine(line.drawing_type, this.selectedLine.data, line.id);
    },
    scrollToLatestCandle() {
      if (this.chartStore.chart) {
        this.chartStore.chart.timeScale().fitContent();
      }
    },
    saveDrawingLine(drawing_type, drawing_data, id = "") {
      const message = {
        event_type: "save_drawing",
        data: {
          drawing_type: drawing_type,
          symbol: this.chartStore.symbol,
          drawing_data: drawing_data,
        },
      };
      if (id) {
        message.data.id = id;
      }
      if (this.chartStore.websocket) {
        this.chartStore.websocket.send(JSON.stringify(message));
      }
    },
    loadDrawingLines() {
      const message = {
        event_type: "load_drawings",
        data: {
          symbol: this.chartStore.symbol,
          drawing_type: "drawing.line",
        },
      };
      if (this.chartStore.websocket) {
        this.chartStore.websocket.send(JSON.stringify(message));
      }
      message.data.drawing_type = "drawing.ray";
      if (this.chartStore.websocket) {
        this.chartStore.websocket.send(JSON.stringify(message));
      }
      message.data.drawing_type = "drawing.trendline";
      if (this.chartStore.websocket) {
        this.chartStore.websocket.send(JSON.stringify(message));
      }
    },
    deleteDrawingLine(drawing_type, id) {
      const message = {
        event_type: "delete_drawing",
        data: {
          drawing_type: drawing_type,
          symbol: this.chartStore.symbol,
          id: id,
        },
      };
      if (this.chartStore.websocket) {
        this.chartStore.websocket.send(JSON.stringify(message));
      }
    },
    async setupWebSocket() {
      return new Promise((resolve) => {
        const websocket = new WebSocket("ws://127.0.0.1:3000/ws");
        this.chartStore.setWebsocket(websocket);
        websocket.onopen = () => {
          this.subscribe();
          resolve();
        };
        websocket.onmessage = (event) => {
          const message = JSON.parse(event.data);
          if (message.event_type === "kline") {
            this.handleWebSocketMessage(message);
          } else if (message.event_type === "drawing_saved") {
            console.log("Уровень сохранён:", message.status);
            if (message.status === "success" && message.id) {
              const line = this.chartStore.drawnLines.find(l => l.id === "");
              if (line) {
                this.chartStore.updateDrawnLine(this.chartStore.drawnLines.findIndex(l => l.id === ""), {
                  ...line,
                  id: message.id,
                });
              }
            }
          } else if (message.event_type === "drawings_loaded") {
            message.data.forEach(({ id, drawing_type, symbol, data }) => {
              const drawingData = JSON.parse(data);
              const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
              if (candlestickObj && candlestickObj.visible) {
                if (drawing_type === "drawing.line") {
                  const line = candlestickObj.series.createPriceLine({
                    price: drawingData.price,
                    color: drawingData.color || "#FFD700",
                    lineWidth: drawingData.line_width || 1,
                    lineStyle: LineStyle.Dashed,
                  });
                  this.chartStore.addDrawnLine({
                    id,
                    drawing_type,
                    data: drawingData,
                    line,
                  });
                } else if (drawing_type === "drawing.ray") {
                  const raySeries = this.chartStore.chart.addLineSeries({
                    color: drawingData.color || "#FFD700",
                    lineWidth: drawingData.line_width || 1,
                    lineStyle: LineStyle.Dashed,
                    priceLineVisible: false,
                    lastValueVisible: false,
                  });
                  raySeries.setData([
                    { time: drawingData.start_time / 1000, value: drawingData.price },
                    { time: this.getLastVisibleTime(), value: drawingData.price },
                  ]);
                  this.chartStore.addDrawnLine({
                    id,
                    drawing_type,
                    data: drawingData,
                    line: raySeries,
                  });
                } else if (drawing_type === "drawing.trendline") {
                  const trendlineSeries = this.chartStore.chart.addLineSeries({
                    color: drawingData.color || "#FFD700",
                    lineWidth: drawingData.line_width || 1,
                    lineStyle: LineStyle.Solid,
                    priceLineVisible: false,
                    lastValueVisible: false,
                  });
                  trendlineSeries.setData([
                    { time: drawingData.start_time / 1000, value: drawingData.start_price },
                    { time: drawingData.end_time / 1000, value: drawingData.end_price },
                  ]);
                  this.chartStore.addDrawnLine({
                    id,
                    drawing_type,
                    data: drawingData,
                    line: trendlineSeries,
                  });
                }
              }
            });
          } else if (message.event_type === "drawing_added") {
            const { id, drawing_type, symbol, data } = message.data;
            const drawingData = JSON.parse(data);
            const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
            if (candlestickObj && candlestickObj.visible && !this.chartStore.drawnLines.find(l => l.id === id)) {
              if (drawing_type === "drawing.line") {
                const line = candlestickObj.series.createPriceLine({
                  price: drawingData.price,
                  color: drawingData.color || "#FFD700",
                  lineWidth: drawingData.line_width || 1,
                  lineStyle: LineStyle.Dashed,
                });
                this.chartStore.addDrawnLine({
                  id,
                  drawing_type,
                  data: drawingData,
                  line,
                });
              } else if (drawing_type === "drawing.ray") {
                const raySeries = this.chartStore.chart.addLineSeries({
                  color: drawingData.color || "#FFD700",
                  lineWidth: drawingData.line_width || 1,
                  lineStyle: LineStyle.Dashed,
                  priceLineVisible: false,
                  lastValueVisible: false,
                });
                raySeries.setData([
                  { time: drawingData.start_time / 1000, value: drawingData.price },
                  { time: this.getLastVisibleTime(), value: drawingData.price },
                ]);
                this.chartStore.addDrawnLine({
                  id,
                  drawing_type,
                  data: drawingData,
                  line: raySeries,
                });
              } else if (drawing_type === "drawing.trendline") {
                const trendlineSeries = this.chartStore.chart.addLineSeries({
                  color: drawingData.color || "#FFD700",
                  lineWidth: drawingData.line_width || 1,
                  lineStyle: LineStyle.Solid,
                  priceLineVisible: false,
                  lastValueVisible: false,
                });
                trendlineSeries.setData([
                  { time: drawingData.start_time / 1000, value: drawingData.start_price },
                  { time: drawingData.end_time / 1000, value: drawingData.end_price },
                ]);
                this.chartStore.addDrawnLine({
                  id,
                  drawing_type,
                  data: drawingData,
                  line: trendlineSeries,
                });
              }
            }
          } else if (message.event_type === "drawing_deleted") {
            console.log("Уровень удалён из базы данных:", message.status);
            const { id } = message.data;
            const line = this.chartStore.drawnLines.find(l => l.id === id);
            if (line) {
              this.removeDrawnLine(line);
            }
          }
        };
        websocket.onerror = (error) => {
          console.error("WebSocket ошибка:", error);
        };
        websocket.onclose = () => {
          console.log("WebSocket закрыт");
        };
      });
    },
    subscribe() {
      const subscription = {
        event_type: "subscribe",
        symbol: this.chartStore.symbol,
        interval: this.chartStore.interval,
        streams: ["kline"],
      };
      if (this.chartStore.websocket) {
        this.chartStore.websocket.send(JSON.stringify(subscription));
      }
    },
    handleWebSocketMessage(message) {
      if (message.kline) {
        const kline = message.kline;
        const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
        const volumeObj = this.chartStore.chartObjects.find((obj) => obj.id === "volume");

        if (candlestickObj && candlestickObj.visible) {
          const candle = {
            time: kline.start_time / 1000,
            open: parseFloat(kline.open),
            high: parseFloat(kline.high),
            low: parseFloat(kline.low),
            close: parseFloat(kline.close),
          };
          candlestickObj.series.update(candle);
          this.chartStore.setLastCandle(candle);
        }

        if (volumeObj && volumeObj.visible) {
          const volume = {
            time: kline.start_time / 1000,
            value: parseFloat(kline.volume),
            color: parseFloat(kline.close) >= parseFloat(kline.open) ? "#26a69a" : "#ef5350",
          };
          volumeObj.series.update(volume);
          this.updateVolumeLabel();
        }
      }
    },
    requestHistoricalData() {
      // Реализация загрузки исторических данных
      // Можно оставить как есть или обновить при необходимости
    },
    loadMoreHistoricalData() {
      // Реализация подгрузки исторических данных
      // Можно оставить как есть или обновить при необходимости
    },
  },
};
</script>