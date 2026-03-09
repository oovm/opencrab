<template>
  <div class="role-detail">
    <div class="role-header mb-8">
      <div class="flex items-center gap-6">
        <img :src="role.avatar" :alt="role.title" class="w-24 h-24 rounded-full object-cover" />
        <div>
          <h2 class="text-3xl font-bold mb-2">{{ role.title }}</h2>
          <div class="flex items-center gap-4">
            <div class="flex items-center gap-2">
              <el-rate v-model="averageRating" disabled show-score />
              <span class="text-gray-500">({{ ratings.length }} 评价)</span>
            </div>
            <span class="text-gray-500">{{ role.tokenLimit.toLocaleString() }} tokens</span>
          </div>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
      <div class="lg:col-span-2">
        <div class="bg-white rounded-2xl border border-gray-200 shadow-sm p-6 mb-6">
          <h3 class="text-xl font-semibold mb-4">职位描述</h3>
          <p class="text-gray-600">{{ role.description }}</p>
        </div>

        <div class="bg-white rounded-2xl border border-gray-200 shadow-sm p-6 mb-6">
          <h3 class="text-xl font-semibold mb-4">技能</h3>
          <div class="flex flex-wrap gap-2">
            <el-tag v-for="skill in role.skills" :key="skill.id" type="info">
              {{ skill.name }}
            </el-tag>
          </div>
        </div>

        <div class="bg-white rounded-2xl border border-gray-200 shadow-sm p-6">
          <div class="flex justify-between items-center mb-4">
            <h3 class="text-xl font-semibold">评论</h3>
            <el-button type="primary" size="small" @click="showReviewDialog = true">
              发表评论
            </el-button>
          </div>
          <div v-if="reviews.length === 0" class="text-center text-gray-500 py-8">
            暂无评论
          </div>
          <div v-else class="space-y-4">
            <div v-for="review in reviews" :key="review.id" class="border-b border-gray-100 pb-4 last:border-0">
              <div class="flex items-center gap-2 mb-2">
                <el-rate :model-value="review.rating?.score || 0" disabled size="small" />
                <span class="text-gray-500 text-sm">{{ new Date(review.createdAt).toLocaleDateString() }}</span>
              </div>
              <p class="text-gray-700">{{ review.content }}</p>
            </div>
          </div>
        </div>
      </div>

      <div class="space-y-6">
        <div class="bg-white rounded-2xl border border-gray-200 shadow-sm p-6">
          <h3 class="text-xl font-semibold mb-4">我的标签</h3>
          <div class="mb-4">
            <div class="flex flex-wrap gap-2 mb-4">
              <el-tag
                v-for="tag in userTags"
                :key="tag.id"
                closable
                @close="removeTag(tag)"
              >
                {{ tag.name }}
              </el-tag>
              <span v-if="userTags.length === 0" class="text-gray-400">还没有标签</span>
            </div>
            <div class="flex gap-2">
              <el-input v-model="newTagName" placeholder="添加标签" size="small" @keyup.enter="addTag" />
              <el-button type="primary" size="small" @click="addTag" :disabled="userTags.length >= 5">
                添加
              </el-button>
            </div>
            <p v-if="userTags.length >= 5" class="text-red-500 text-sm mt-2">最多只能添加 5 个标签</p>
          </div>
        </div>
      </div>
    </div>

    <el-dialog v-model="showReviewDialog" title="发表评论" width="500px">
      <el-form :model="reviewForm" label-position="top">
        <el-form-item label="评分">
          <el-rate v-model="reviewForm.score" />
        </el-form-item>
        <el-form-item label="评论内容">
          <el-input v-model="reviewForm.content" type="textarea" :rows="4" placeholder="分享你的体验..." />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showReviewDialog = false">取消</el-button>
        <el-button type="primary" @click="submitReview">提交</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { ElMessage } from "element-plus";
import { useRoleStore } from "../stores";
import { useAuthStore } from "../stores";
import type { Role, Review, Tag } from "../types";

const props = defineProps<{
    role: Role;
}>();

const roleStore = useRoleStore();
const authStore = useAuthStore();

const ratings = ref(roleStore.getRatingsForRole(props.role.id));
const reviews = ref(roleStore.getReviewsForRole(props.role.id));
const userTags = ref<Tag[]>([]);
const newTagName = ref("");
const showReviewDialog = ref(false);

const reviewForm = ref({
    score: 0,
    content: "",
});

const averageRating = computed(() => roleStore.getAverageRatingForRole(props.role.id));

onMounted(() => {
    loadUserTags();
});

function loadUserTags() {
    if (authStore.user) {
        const userTag = roleStore.getCurrentUserTags(props.role.id);
        userTags.value = userTag?.tags || [];
    }
}

function addTag() {
    if (!newTagName.value.trim()) return;
    if (userTags.value.length >= 5) {
        ElMessage.error("最多只能添加 5 个标签");
        return;
    }

    const newTag: Tag = {
        id: crypto.randomUUID(),
        name: newTagName.value.trim(),
    };

    userTags.value.push(newTag);
    saveUserTags();
    newTagName.value = "";
}

function removeTag(tag: Tag) {
    userTags.value = userTags.value.filter((t) => t.id !== tag.id);
    saveUserTags();
}

function saveUserTags() {
    if (!authStore.user) return;
    try {
        roleStore.setCurrentUserTags(props.role.id, userTags.value);
        ElMessage.success("标签已更新");
    } catch (e: any) {
        ElMessage.error(e.message);
    }
}

function submitReview() {
    if (!authStore.user) {
        ElMessage.error("请先登录");
        return;
    }
    if (!reviewForm.value.content.trim()) {
        ElMessage.error("请输入评论内容");
        return;
    }

    let rating;
    if (reviewForm.value.score > 0) {
        rating = roleStore.createRating({
            userId: authStore.user.id,
            roleId: props.role.id,
            score: reviewForm.value.score,
        });
    }

    roleStore.createReview({
        userId: authStore.user.id,
        roleId: props.role.id,
        content: reviewForm.value.content,
        rating,
    });

    reviews.value = roleStore.getReviewsForRole(props.role.id);
    ratings.value = roleStore.getRatingsForRole(props.role.id);
    showReviewDialog.value = false;
    reviewForm.value = { score: 0, content: "" };
    ElMessage.success("评论已发表");
}
</script>

<style scoped>
.role-detail {
  max-width: 7xl;
  margin: 0 auto;
  padding: 3rem 1rem;
}
</style>
